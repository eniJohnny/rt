use std::{
    cmp::min,
    collections::VecDeque,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread,
    time::Duration,
};

use image::RgbaImage;

use crate::{
    model::{
        materials::Color,
        maths::{quaternion::Quaternion, ray::Ray},
        scene::Scene,
    },
    BASE_SIMPLIFICATION, MAX_THREADS, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::raycasting::cast_ray;

#[derive(Clone)]
struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub factor: usize,
}

fn generate_tiles_for(queue: &Arc<Mutex<VecDeque<Tile>>>, simplification_factor: usize) -> u32 {
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
                    factor,
                });
                y += simplification_factor;
            }
            x += simplification_factor;
        }
        factor /= 2;
    }
    cpt
}

fn get_angle_to(fov: f64, pos: f64, length: f64) -> f64 {
    (pos / length - 0.5) * fov
}

pub fn start_render_threads(
    scene: Arc<RwLock<Scene>>,
) -> (Receiver<(RgbaImage, bool)>, Sender<bool>) {
    // Channel render_thread -> main_thread pour envoyer l'image du thread de render au main thread
    let (ta, ra) = mpsc::channel();
    // Channel main_thread -> render_thread pour recevoir les demandes d'images du main thread, avec ou sans changement de scene
    let (tb, rb) = mpsc::channel();
    // Channel worker_threads[] -> render_thread pour recevoir les tiles renderees par les worker thread qui travaillent constamment en fond
    let (tc, rc) = mpsc::channel();

    // La work_queue contient toutes les tiles en attente de render, dans toutes leurs versions de resolution(de 64x64:1 a 1x1:1)
    // Elle est protegee par un mutex car les worker threads vont constemment prendre du travail de cette queue, et le render thread
    // va en rajouter a chaque demande d'image
    let work_queue = Arc::new(Mutex::new(VecDeque::new()));

    // Reference a la scene thread-safe protegee par un RwLock : une sorte de Mutex qui autorise les threads qui ne font que de la lecture
    // a pouvoir tous consulter de maniere concurrente, tant que personne ne tente de prendre les droits d'ecriture.
    // Seul le main_thread fera des modifications a la scene, le render_thread et les worker_threads ne feront que de la lecture
    let scene = Arc::clone(&scene);

    // Generation du thread de render qui va lui-meme lancer les worker threads
    thread::spawn(move || {
        for _ in 0..MAX_THREADS {
            // Chaque worker_thread a son propre emetteur de channel, mais il n'existe qu'un seul receiver (le render_thread)
            let cur_tx = tc.clone();
            let work_queue_clone = Arc::clone(&work_queue);
            let scene = Arc::clone(&scene);
            thread::spawn(move || loop {
                while let Some(tile) = {
                    let mut wq = work_queue_clone.lock().unwrap();
                    wq.pop_front()
                } {
                    let mut colors = Vec::new();
                    let scene = scene.read().unwrap();
                    let camera = scene.camera();
                    // Pour chaque pixel de cette tile qui n'a pas deja ete calcule par une taille de plus haut factor
                    // Sur une tile 64x64 avec un factor de 64, il n'y aura qu'un ray de lance. Pour un factor de 32
                    // 4 rays seront lancers (4 x 32x32 = 64x64)
                    for_each_uncalculated_pixel(&tile, |x, y| {
                        // On calcule le ray et on le cast
                        let roll = get_angle_to(camera.fov(), x as f64, SCREEN_WIDTH as f64);
                        let pitch = get_angle_to(camera.vfov(), y as f64, SCREEN_HEIGHT as f64);
                        let quat = Quaternion::from_euler_angles(pitch, roll, 0.);
                        let ray = Ray::new(
                            camera.pos().clone(),
                            camera.dir().clone().rotate(&quat).normalize(),
                            0,
                        );
                        colors.push(cast_ray(&scene, &ray));
                    });
                    // Envoi de la tile
                    cur_tx.send((tile, colors)).unwrap();
                }
                // Lorsqu'il n'y a plus de travail de disponible pour le moment, on ne surcharge pas la queue avec des reads constants.
                thread::sleep(Duration::from_millis(5));
            });
        }
        build_image_from_tilesets(rc, rb, ta, work_queue);
    });
    (ra, tb)
}

/**
 * Boucle principale du render thread, qui doit aggreger les tiles rendered par les worker_threads sur une image rgba
 * qu'il se tient pret a tout moment a envoyer au main_thread pour l'affichage. Lorsque la resolution finale (factor = 1)
 * est effectuee,
 */
fn build_image_from_tilesets(
    rc: Receiver<(Tile, Vec<Color>)>,
    rb: Receiver<bool>,
    ta: Sender<(RgbaImage, bool)>,
    work_queue: Arc<Mutex<VecDeque<Tile>>>,
) {
    // Bon c'est un peu le bordel, je pense que je pourrais faire un truc mieux que ca, je previens, la c'est un peu fouillis

    // On remplit la work queue avec les toutes les tiles, pour chaque resolution possible, de la toute premiere image.
    // La fonction renvoie le nombre de tile d'une resolution donnee.
    // Cela nous permet de traquer quand est-ce que l'image de la plus basse resolution possible est completee, car c'est le
    // point ou on peux l'envoyer au main_thread.
    let mut low_res_to_do = generate_tiles_for(&work_queue, BASE_SIMPLIFICATION);
    let mut img = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    loop {
        // On recoit les demandes d'images du main_thread (une seule a la fois, pas de nouvelle demande tant qu'on a pas envoye une image)
        if let Ok(scene_change) = rb.try_recv() {
            // Si la scene a change entre temps depuis le GUI, on reset tout
            if scene_change {
                img = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
                work_queue.lock().unwrap().clear();
                // On vide egalement le channel.
                while let Ok(_) = rc.try_recv() {}
                low_res_to_do = generate_tiles_for(&work_queue, BASE_SIMPLIFICATION);
                //TODO: Actuellement les worker_threads en cours de render vont probablement envoyer la tile qu'ils sont en train de render
                //      au moment du reset. Il faut trouver un moyen de les empecher.
            } else if low_res_to_do == 0 {
                // Si aucun changement n'a ete detecte, et qu'on a AU MOINS une image de la plus basse resolution, on l'envoie
                ta.send((img.clone(), work_queue.lock().unwrap().is_empty()))
                    .unwrap();
            }
        }
        // Reception des tiles render par les worker_threads
        if let Ok((tile, colors)) = rc.try_recv() {
            let mut index = 0;
            // Meme chose que dans render_tilesets, on ne remplit que les zones necessaires par tile et par resolution.
            for_each_uncalculated_pixel(&tile, |x, y| {
                let color = colors[index].clone().to_rgba();
                index += 1;
                for x in x..min(x + &tile.factor, SCREEN_WIDTH) {
                    for y in y..min(y + &tile.factor, SCREEN_HEIGHT) {
                        img.put_pixel(x as u32, y as u32, color);
                    }
                }
            });
            // On retient les tile de la plus basse resolution qui passent par la
            if tile.factor == BASE_SIMPLIFICATION {
                low_res_to_do -= 1;
                if low_res_to_do == 0 {
                    // Aucun autre check necessaire, si on genere une image et qu'on vient a peine de finir la plus basse resolution,
                    // c'est forcement qu'on doit l'envoyer a l'UI.
                    ta.send((img.clone(), false)).unwrap();
                }
            }
        }
    }
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
