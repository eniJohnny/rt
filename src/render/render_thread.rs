use std::{cmp::min, collections::{HashMap, VecDeque}, sync::{mpsc::{Receiver, Sender}, Arc, RwLock}, time::Instant};
use image::RgbaImage;

use crate::{model::{materials::color::Color, scene::Scene}, BASE_SIMPLIFICATION, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

use super::{common::{for_each_uncalculated_pixel, QueueContext, Tile}, settings::ViewMode, tile_queue::fill_work_queue};

pub enum UIOrder {
    SceneChange(usize),
    NewScene(Arc<RwLock<Scene>>),
    ScenePause(usize),
    SceneStart(usize),
    AskImage(usize),
    CloseScene(usize)
}

pub struct SceneRender {
    pub render_id: usize,
    pub scene: Arc<RwLock<Scene>>,
    pub low_res_to_do: u32,
    pub max_res_to_do: u32,
    iterations_done: usize,
    img: Vec<Vec<Color>>,
    pub version: usize
}

/**
 * Boucle principale du render thread, qui doit aggreger les tiles rendered par les worker_threads sur une image rgba
 * qu'il se tient pret a tout moment a envoyer au main_thread pour l'affichage. Lorsque la resolution finale (factor = 1)
 * est effectuee,
 */
pub fn render_thread(
    rc: Receiver<(Tile, Vec<Color>)>,
    rb: Receiver<UIOrder>,
    ta: Sender<(RgbaImage, bool)>,
    work_queue: Arc<RwLock<Vec<QueueContext>>>
) {
    let mut render_list: HashMap<usize, SceneRender> = HashMap::new();
    let mut next_render_id: usize = 0;
    // Bon c'est un peu le bordel, je pense que je pourrais faire un truc mieux que ca, je previens, la c'est un peu fouillis

    // On remplit la work queue avec toutes les tiles, pour chaque resolution possible, de la toute premiere image.
    // La fonction renvoie le nombre de tile d'une resolution donnee.
    // Cela nous permet de traquer quand est-ce que l'image de la plus basse resolution possible est completee, car c'est le
    // point ou on peux l'envoyer au main_thread.

    let mut asked_image = None;
    let mut perf = Instant::now();

    loop {
        // On recoit les demandes d'images du main_thread (une seule a la fois, pas de nouvelle demande tant qu'on a pas envoye une image)
        if let Ok(ui_order) = rb.try_recv() {
            match ui_order {
                UIOrder::AskImage(scene_id) => {
                    if let Some(_) = render_list.get_mut(&scene_id) { 
                        asked_image = Some(scene_id);
                    }
                },
                UIOrder::SceneChange(scene_id) => {
                    if let Some(render) = render_list.get_mut(&scene_id) {
                        render.img = vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH];
                        render.iterations_done = 0;
                        render.version += 1;

                        for queue_context in work_queue.write().unwrap().iter_mut() {
                            queue_context.avg_time = 0.;
                            queue_context.tiles_done = 0.;
                        }
                        fill_work_queue(render, &work_queue, min(BASE_SIMPLIFICATION, TILE_SIZE));
                    }
                },
                UIOrder::NewScene(scene) => {
                    create_scene_render(scene, &mut render_list, &mut next_render_id, &work_queue);
                },
                UIOrder::SceneStart(scene_id) => {
                    if let Some(render) = render_list.get_mut(&scene_id) {
                        for queue in work_queue.write().unwrap().iter_mut() {
                            if queue.render_id == render.render_id && !queue.active {
                                queue.active = true;
                            }
                        }
                    }
                },
                UIOrder::ScenePause(scene_id) => {
                    if let Some(render) = render_list.get_mut(&scene_id) {
                        for queue in work_queue.write().unwrap().iter_mut() {
                            if queue.render_id == render.render_id && queue.active {
                                queue.active = false;
                            }
                        }
                    }
                },
                UIOrder::CloseScene(scene_id) => {
                    let mut work_queue = work_queue.write().unwrap();
                    let mut index = 0;
                    for queue in work_queue.iter() {
                        if queue.render_id == scene_id {
                            break;
                        }
                        index += 1;
                    }
                    if index != work_queue.len() {
                        work_queue.remove(index);
                    }
                    render_list.remove(&scene_id);
                }
            }
            // Si la scene a change entre temps depuis le GUI, on reset tout
        }
        loop  {
            // Reception des tiles render par les worker_threads
            if let Ok((tile, colors)) = rc.try_recv() {
                if let Some(mut render) = render_list.get_mut(&tile.render_id) {
                    if render.version == tile.version {
                        let mut index = 0;
                        // Meme chose que dans render_tilesets, on ne remplit que les zones necessaires par tile et par resolution.
                        for_each_uncalculated_pixel(&tile, |x, y| {
                            let color = (&colors[index]).clone();
                            index += 1;
                            // Si on a passer la premiere iteration, on ne remplit plus inutilement les images avec un facteur simplifie
                            let range = match render.iterations_done {
                                0 => tile.factor,
                                _ => 1
                            };

                            for x in x..min(x + range, SCREEN_WIDTH) {
                                for y in y..min(y + range, SCREEN_HEIGHT) {
                                    if render.iterations_done > 0 {
                                        // Si on est en train d'iterer, on agrege la couleur recue sur l'image en fonction du nombre d'iteration
                                        let mut base_color = render.img.get(x).unwrap().get(y).unwrap().clone();
                                        let iterations_done = render.iterations_done as f64;
                                        base_color = (base_color * (iterations_done) / (iterations_done + 1.))
                                            + (&color * (1. / (iterations_done + 1.)));
                                        render.img.get_mut(x).unwrap()[y] = base_color;
                                    } else {
                                        // Sinon, on la prend telle quelle
                                        let vec_mut = render.img.get_mut(x).unwrap().get_mut(y).unwrap();
                                        color.clone_into(vec_mut);
                                    }
                                }
                            }
                        });
                        // On retient les tile de la plus basse resolution qui passent par la
                        if tile.factor == min(BASE_SIMPLIFICATION, TILE_SIZE) && render.low_res_to_do > 0 {
                            render.low_res_to_do -= 1;
                        }
                        if tile.factor == 1 && render.max_res_to_do > 0 {
                            render.max_res_to_do -= 1;
                        }
    
                        if render.max_res_to_do == 0 {
                            let viewmode = render.scene.read().unwrap().settings().view_mode.clone();
                            match viewmode {
                                ViewMode::HighDef => {
                                    render.iterations_done += 1;
                                    if render.iterations_done < render.scene.read().unwrap().settings().iterations
                                    {
                                        fill_work_queue(&mut render, &work_queue, 1);
                                    }
                                    println!("{} iterations done - {:?}", render.iterations_done, perf.elapsed());
                                    perf = Instant::now();
                                }
                                _ => {}
                            }
                        }

                    }
                }
            }

            if let Some(asked_render_id) = asked_image {
                if let Some(render) = render_list.get_mut(&asked_render_id) {
                    if render.iterations_done != 0 || render.low_res_to_do == 0 {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        if asked_image.is_some() {
            if let Some(render) = render_list.get_mut(&asked_image.unwrap()) {
                let viewmode = render.scene.read().unwrap().settings().view_mode.clone();
                // Si aucun changement n'a ete detecte on envoie l'image actuelle
                let mut img = vec_to_image(&render.img);
                let filter = render.scene.read().unwrap().settings().filter;
                filter.apply(&mut img);
                match viewmode {
                    ViewMode::HighDef => {
                        ta.send((
                            img,
                            render.iterations_done == render.scene.read().unwrap().settings().iterations,
                        ))
                        .ok();
                    }
                    _ => {
                        ta.send((img, false)).ok();
                    }
                }
                asked_image = None;
            } else {
                println!("Render {} has not been found", asked_image.unwrap());
            }
        }
    }
}

fn create_scene_render(scene: Arc<RwLock<Scene>>, render_list: &mut HashMap<usize, SceneRender>, next_render_id: &mut usize, queues: &Arc<RwLock<Vec<QueueContext>>>) {
    let render = SceneRender {
        render_id: *next_render_id,
        low_res_to_do: 0,
        max_res_to_do: 0,
        iterations_done: 0,
        img: vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH],
        version: 0,
        scene : scene.clone()
    };

    queues.write().unwrap().push(QueueContext{
        queue: VecDeque::new(),
        active: true,
        render_id: *next_render_id,
        avg_time: 0.,
        tiles_done: 0.
    });
    render_list.insert(*next_render_id, render);
    *next_render_id += 1;
}

fn vec_to_image(vec: &Vec<Vec<Color>>) -> RgbaImage {
    let mut image = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let mut vec = vec[x][y].clone();
            vec.apply_gamma();
            image.put_pixel(x as u32, y as u32, vec.to_rgba());
        }
    }

    image
}