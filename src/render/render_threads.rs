use std::{
    cmp::min,
    collections::VecDeque,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use image::{GenericImageView, Rgba, RgbaImage};

use crate::{
    model::{
        materials::color::{self, Color},
        scene::Scene,
    },
    BASE_SIMPLIFICATION, MAX_ITERATIONS, MAX_THREADS, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::{
    lighting::{lighting_real::get_lighting_from_ray, simple::simple_lighting_from_ray},
    raycasting::{get_closest_hit, get_ray, sampling_ray},
    restir::PathBucket, settings::ViewMode,
};

#[derive(Clone)]
struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub factor: usize
}

fn generate_tiles_for(
    queue: &Arc<Mutex<VecDeque<Tile>>>,
    sampling: bool,
    simplification_factor: usize,
) -> u32 {
    let mut cpt = 0;
    let mut factor = simplification_factor;
    // Pour chaque resolution possible on genere les demandes de render de tiles, en commencant
    // par la resolution la plus basse pour qu'elle soit render en premier.
    // On a beau tout envoyer d'un coup dans la queue, les worker_threads sont eveilles pendant ce temps
    // et prennent les ordres au fur et a mesure qu'ils sont ajoutes ce qui evite une perte de performance.
    while factor != 0 {
        let mut x = 0;
        while x < SCREEN_WIDTH {
            let mut y = 0;
            while y < SCREEN_HEIGHT {
                let width = min(SCREEN_WIDTH - x, simplification_factor);
                let height = min(SCREEN_HEIGHT - y, simplification_factor);
                if factor == 1 {
                    cpt += 1;
                }
                queue.lock().unwrap().push_back(Tile {
                    x,
                    y,
                    width,
                    height,
                    factor
                });
                y += simplification_factor;
            }
            x += simplification_factor;
        }
        factor /= 2;
    }
    cpt
}

pub fn start_render_threads(
    scene: Arc<RwLock<Scene>>,
) -> (Receiver<(RgbaImage, bool)>, Sender<bool>) {
    // Channel render_thread -> main_thread pour envoyer l'image du thread de render au main thread
    let (ta, ra) = mpsc::channel();
    // Channel main_thread -> render_thread pour recevoir les demandes d'images du main thread, avec ou sans changement de scene
    let (tb, rb) = mpsc::channel();
    // Channels worker_threads[] -> render_thread pour recevoir les tiles renderees par les worker thread qui travaillent constamment en fond
    // Communication de couleur finale des pixels
    let (tc, rc) = mpsc::channel();
    // Communication des echantillons des pixels pour ReSTIR
    // let (td, rd) = mpsc::channel();

    // La work_queue contient toutes les tiles en attente de render, dans toutes leurs versions de resolution(de 64x64:1 a 1x1:1)
    // Elle est protegee par un mutex car les worker threads vont constemment prendre du travail de cette queue, et le render thread
    // va en rajouter a chaque demande d'image
    let work_queue = Arc::new(Mutex::new(VecDeque::<Tile>::new()));

    // Reference a la scene thread-safe protegee par un RwLock : une sorte de Mutex qui autorise les threads qui ne font que de la lecture
    // a pouvoir tous consulter de maniere concurrente, tant que personne ne tente de prendre les droits d'ecriture.
    // Seul le main_thread fera des modifications a la scene, le render_thread et les worker_threads ne feront que de la lecture
    let scene = Arc::clone(&scene);

    // Generation du thread de render qui va lui-meme lancer les worker threads
    thread::spawn(move || {
        for _ in 0..MAX_THREADS {
            // Chaque worker_thread a son propre emetteur de channel, mais il n'existe qu'un seul receiver (le render_thread)
            let cur_tx = tc.clone();
            // let cur_td = td.clone();
            let work_queue_clone = Arc::clone(&work_queue);
            let scene = Arc::clone(&scene);
            thread::spawn(move || loop {
                while let Some(tile) = {
                    let mut wq = work_queue_clone.lock().unwrap();
                    wq.pop_front()
                } {
                    let scene = scene.read().unwrap();
                    // Pour chaque pixel de cette tile qui n'a pas deja ete calcule par une taille de plus haut factor
                    // Sur une tile 64x64 avec un factor de 64, il n'y aura qu'un ray de lance. Pour un factor de 32
                    // 4 rays seront lancers (4 x 32x32 = 64x64)

                    // let mut buckets = Vec::new();
                    let mut colors = Vec::new();
                    for_each_uncalculated_pixel(&tile, |x, y| {
                        // On calcule le ray et on le cast
                        let ray = get_ray(&scene, x, y);
                        if let ViewMode::Simple(ambient, light) = &scene.settings().view_mode {
                            colors.push(simple_lighting_from_ray(&scene, &ray, ambient, light))
                        } else {
                            colors.push(get_lighting_from_ray(&scene, &ray))
                        }
                    });
                    cur_tx.send((tile, colors)).ok();
                }
                thread::sleep(Duration::from_millis(10));
                // Lorsqu'il n'y a plus de travail de disponible pour le moment, on ne surcharge pas la queue avec des reads constants.
            });
        }
        main_render_loop(rc, rb, ta, work_queue, scene);
    });
    (ra, tb)
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

fn fill_work_queue(low_res_to_do: &mut u32, max_res_to_do: &mut u32, nb_tiles_to_do: &mut usize, queue: &Arc<Mutex<VecDeque<Tile>>>,simplification_factor: usize, rc: &Receiver<(Tile, Vec<Color>)>) {

    let nb_tiles_left;
    {
        let mut queue = queue.lock().unwrap();
        nb_tiles_left = queue.len();
        queue.clear();
    }
    // On vide egalement le channel.
    let mut nb_tiles_being_rendered = *nb_tiles_to_do - nb_tiles_left;
    while nb_tiles_being_rendered > 0 {
        if let Ok(_) = rc.try_recv() {
            nb_tiles_being_rendered -= 1;
        }
    }
    // thread::sleep(Duration::from_millis(20));

    *low_res_to_do = generate_tiles_for(
        &queue,
        false,
        simplification_factor,
    );
    *max_res_to_do = *low_res_to_do;
    let mut factor = simplification_factor;
    *nb_tiles_to_do = 0;
    loop {
        *nb_tiles_to_do += *low_res_to_do as usize;
        if factor == 1 {
            break;
        }
        factor /= 2;
    }
}

/**
 * Boucle principale du render thread, qui doit aggreger les tiles rendered par les worker_threads sur une image rgba
 * qu'il se tient pret a tout moment a envoyer au main_thread pour l'affichage. Lorsque la resolution finale (factor = 1)
 * est effectuee,
 */
fn main_render_loop(
    rc: Receiver<(Tile, Vec<Color>)>,
    rb: Receiver<bool>,
    ta: Sender<(RgbaImage, bool)>,
    work_queue: Arc<Mutex<VecDeque<Tile>>>,
    scene: Arc<RwLock<Scene>>,
) {
    // Bon c'est un peu le bordel, je pense que je pourrais faire un truc mieux que ca, je previens, la c'est un peu fouillis

    // On remplit la work queue avec toutes les tiles, pour chaque resolution possible, de la toute premiere image.
    // La fonction renvoie le nombre de tile d'une resolution donnee.
    // Cela nous permet de traquer quand est-ce que l'image de la plus basse resolution possible est completee, car c'est le
    // point ou on peux l'envoyer au main_thread.
    let mut low_res_to_do = 0;
    let mut max_res_to_do = 0;
    let mut nb_tiles_to_do = 0;
    fill_work_queue(&mut low_res_to_do, &mut max_res_to_do, &mut nb_tiles_to_do, &work_queue, BASE_SIMPLIFICATION, &rc);

    let mut iterations_done = 0;
    let mut img = vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH];
    let mut final_img = vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH];
    let mut to_send = false;
    let mut perf = Instant::now();

    loop {
        loop {
            // Reception des tiles render par les worker_threads
            if let Ok((tile, colors)) = rc.try_recv() {
                let mut index = 0;
                // Meme chose que dans render_tilesets, on ne remplit que les zones necessaires par tile et par resolution.
                for_each_uncalculated_pixel(&tile, |x, y| {
                    let mut color = (&colors[index]).clone();
                    // color.apply_gamma();
                    index += 1;
                    for x in x..min(x + &tile.factor, SCREEN_WIDTH) {
                        for y in y..min(y + &tile.factor, SCREEN_HEIGHT) {
                            let vec_mut = img.get_mut(x).unwrap().get_mut(y).unwrap();
                            color.clone_into(vec_mut);
                        }
                    }
                });
                nb_tiles_to_do -= 1;
                // On retient les tile de la plus basse resolution qui passent par la
                if tile.factor == BASE_SIMPLIFICATION && low_res_to_do > 0 {
                    low_res_to_do -= 1;
                } else if tile.factor == 1 && max_res_to_do > 0 {
                    max_res_to_do -= 1;
                }

                if max_res_to_do == 0 {
                    let viewmode = scene.read().unwrap().settings().view_mode.clone();
                    match viewmode {
                        ViewMode::HighDef => {
                            iterations_done += 1;
                            final_img = add_iteration_to_final_img(img, final_img, iterations_done);
                            if iterations_done < scene.read().unwrap().settings().iterations as i32
                            {
                                fill_work_queue(&mut low_res_to_do, &mut max_res_to_do, &mut nb_tiles_to_do, &work_queue, BASE_SIMPLIFICATION, &rc);
                            }
                            img = vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH];
                            println!("{} iterations done - {:?}", iterations_done, perf.elapsed());
                            perf = Instant::now();
                        }
                        _ => {}
                    }
                }
            }
            let viewmode = scene.read().unwrap().settings().view_mode.clone();
            if let ViewMode::HighDef = viewmode {
                if max_res_to_do == 0
                    && iterations_done < scene.read().unwrap().settings().iterations as i32
                {
                    fill_work_queue(&mut low_res_to_do, &mut max_res_to_do, &mut nb_tiles_to_do, &work_queue, BASE_SIMPLIFICATION, &rc);
                }
            }
            if low_res_to_do == 0 {
                break;
            }
        }

        if to_send {
            let viewmode = scene.read().unwrap().settings().view_mode.clone();
            // Si aucun changement n'a ete detecte on envoie l'image actuelle
            match viewmode {
                ViewMode::HighDef => {
                    if iterations_done > 0 {
                        ta.send((
                            vec_to_image(&final_img),
                            iterations_done == scene.read().unwrap().settings().iterations as i32,
                        ))
                        .ok();
                    } else {
                        ta.send((vec_to_image(&img), false)).ok();
                    }
                }
                _ => {
                    ta.send((vec_to_image(&img), false)).ok();
                }
            }
            to_send = false;
        }
        // On recoit les demandes d'images du main_thread (une seule a la fois, pas de nouvelle demande tant qu'on a pas envoye une image)
        if let Ok(scene_change) = rb.try_recv() {
            // Si la scene a change entre temps depuis le GUI, on reset tout
            if scene_change {
                img = vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH];
                final_img = vec![vec![Color::new(0., 0., 0.); SCREEN_HEIGHT]; SCREEN_WIDTH];
                iterations_done = 0;
                fill_work_queue(&mut low_res_to_do, &mut max_res_to_do, &mut nb_tiles_to_do, &work_queue, BASE_SIMPLIFICATION, &rc);
            } else {
                to_send = true;
            }
        }
    }
}

fn add_iteration_to_final_img(
    iteration: Vec<Vec<Color>>,
    mut final_img: Vec<Vec<Color>>,
    iterations_done: i32,
) -> Vec<Vec<Color>> {
    if iterations_done > 1 {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let mut base_color = final_img.get(x).unwrap().get(y).unwrap().clone();
                let mut new_iter_color = iteration.get(x).unwrap().get(y).unwrap();
                let iterations_done = iterations_done as f64;
                base_color = (base_color * (iterations_done - 1.) / iterations_done)
                    + (new_iter_color * (1. / iterations_done as f64));
                final_img.get_mut(x).unwrap()[y] = base_color;
            }
        }
    } else {
        return iteration;
    }
    final_img
}

fn for_each_uncalculated_pixel<F>(tile: &Tile, mut f: F)
where
    F: FnMut(usize, usize),
{
    if tile.factor == BASE_SIMPLIFICATION {
        f(tile.x, tile.y)
    } else {
        let mut offset_x = 0;
        let old_factor = tile.factor * 2;
        while offset_x < tile.width {
            let mut offset_y = 0;
            while offset_y < tile.height {
                let x = tile.x + offset_x;
                let y = tile.y + offset_y;
                f(x + tile.factor, y);
                f(x, y + tile.factor);
                f(x + tile.factor, y + tile.factor);
                offset_y += old_factor;
            }
            offset_x += old_factor;
        }
    }
}
