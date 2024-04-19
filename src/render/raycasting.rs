use std::{
    borrow::Borrow,
    cmp::min,
    collections::VecDeque,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use image::RgbaImage;

use crate::{
    model::{
        materials::Color,
        maths::{hit::Hit, quaternion::Quaternion, ray::Ray, vec3::Vec3},
        objects::{camera::Camera, light::AmbientLight},
        scene::Scene,
        Element,
    },
    BASE_SIMPLIFICATION, MAX_THREADS, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::lighting::apply_lighting;

#[derive(Clone)]
struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub factor: usize,
}

pub fn render_scene(scene: &Scene) -> RgbaImage {
    render_scene_tilesets(scene)
    // render_scene_threadpool(scene)
}

/**
 * Generation d'une matrice de ray selon les vecteurs camera.
 * Pour ca, on projete un ecran devant la camera a cam.pos + cam.dir, et passe les rays a travers tous les pixels de cet ecran.
 */
// pub fn generate_rays(camera: &mut Camera) {
//     // U et V sont les vecteurs unitaires de l'ecran projete.
//     let u = Vec3::new(*camera.dir().z(), 0., - *camera.dir().x()).normalize();
//     let v = - camera.dir().cross(&u).normalize();
//     // Tailles de l'ecran
//     let width = (camera.fov()/2.).tan() * 2.;
//     let height = width * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
//     // Centre de l'ecran
//     let center: Vec3 = camera.pos() + camera.dir();

//     // Coin superieur gauche, et les distances pour atteindre a partir de lui les coin superieur droit et inferieur gauche
//     let top_left = center +  &u * - width/2. - &v * height/2.;
//     let left_to_right = &u * width;
//     let top_to_bot = v * height;

//     let mut result: Vec<Vec<Ray>> = vec![];
//     for x in 0..SCREEN_WIDTH {
//         let mut line: Vec<Ray> = vec![];
//         for y in 0..SCREEN_HEIGHT {
//             let pos = camera.pos().clone();
//             // Du coup on fait le ratio entre sa position x, y et la taille en pixel de l'ecran, et on applique une
//             // translation a partir du coing en haut a gauche pour nous donner le ray
//             let dir = &top_left + &top_to_bot * (y as f64 / SCREEN_HEIGHT as f64) + &left_to_right * (x as f64 / SCREEN_WIDTH as f64) - &pos;
//             let ray = Ray::new(pos, dir.normalize(), 0);
//             line.push(ray);
//         }
//         result.push(line);
//     }
// }

fn get_angle_to(fov: f64, pos: f64, length: f64) -> f64 {
    (pos / length - 0.5) * fov
}

fn get_unit_quats(camera: &Camera, res_x: f64) -> (Quaternion, Quaternion) {
    let angle = camera.fov() / res_x;
    let x_quat = Quaternion::new_from_axis_angle(camera.v(), angle);
    let y_quat = Quaternion::new_from_axis_angle(camera.u(), angle);
    (x_quat, y_quat)
}

pub fn generate_tiles_for(scene: &Scene, mut simplification_factor: usize) -> VecDeque<Tile> {
    let mut work_queue = VecDeque::new();
    let mut x = 0;
    while x < SCREEN_WIDTH {
        let mut y = 0;
        while y < SCREEN_HEIGHT {
            let mut factor = simplification_factor;
            let width = min(SCREEN_WIDTH - x, simplification_factor);
            let height = min(SCREEN_HEIGHT - y, simplification_factor);
            while factor != 0 {
                work_queue.push_back(Tile {
                    x,
                    y,
                    width,
                    height,
                    factor,
                });
                factor /= 2;
            }
            y += simplification_factor;
        }
        x += simplification_factor;
    }
    work_queue
}

pub fn render_scene_tilesets(scene: &Scene) -> RgbaImage {
    let (tx, rx) = mpsc::channel();
    /* Pour ne pas join a la main et pouvoir partager des reference immutables entre les threads, on declare un scope dans lequel
     * on spawn nos threads. Le scope s'assure que tout thread qui a ete spawn a l'interieur de ce scope est join avant la fin du scope. */
    thread::scope(|s| {
        let work_queue = Arc::new(Mutex::new(generate_tiles_for(scene, BASE_SIMPLIFICATION)));
        // 16 threads pour 16 core
        for id_thread in 0..MAX_THREADS {
            let cur_tx = tx.clone();
            let work_queue_clone = Arc::clone(&work_queue);
            s.spawn(move || {
                let camera = scene.camera();

                while let Some(mut tile) = {
                    let mut wq = work_queue_clone.lock().unwrap();
                    wq.pop_front()
                } {
                    let mut colors = Vec::new();
                    for_each_uncalculated_pixel(&tile, |x, y| {
                        let roll = get_angle_to(camera.fov(), x as f64, SCREEN_WIDTH as f64);
                        let pitch = get_angle_to(camera.vfov(), y as f64, SCREEN_HEIGHT as f64);
                        let quat = Quaternion::from_euler_angles(pitch, roll, 0.);
                        let ray = Ray::new(
                            camera.pos().clone(),
                            camera.dir().clone().rotate(&quat).normalize(),
                            0,
                        );
                        colors.push(cast_ray(scene, &ray));
                    });
                    cur_tx.send((tile.clone(), colors)).unwrap();
                }
            });
        }
        build_image_from_tilesets(rx, work_queue)
    })
}

pub fn build_image_from_tilesets(
    rx: Receiver<(Tile, Vec<Color>)>,
    work_queue: Arc<Mutex<VecDeque<Tile>>>,
) -> RgbaImage {
    let mut img = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let mut cpt = 0;
    while cpt < 375 {
        if let Ok((tile, colors)) = rx.try_recv() {
            // println!("Starting tile {} {} print at factor {}", tile.x, tile.y, tile.factor);
            let mut index = 0;
            for_each_uncalculated_pixel(&tile, |x, y| {
                let color = colors[index].clone().to_rgba();
                index += 1;
                for x in x..min(x + &tile.factor, SCREEN_WIDTH) {
                    for y in y..min(y + &tile.factor, SCREEN_HEIGHT) {
                        img.put_pixel(x as u32, y as u32, color);
                    }
                }
            });
            if tile.factor == 1 {
                cpt += 1;
            }
        }
        // thread::sleep(Duration::from_millis(3));
        // println!("Index {}", cpt);
    }
    img
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

/* Divise la charge de travails entre 16 threads, et leur donne chacun un nombre de colonnes a calculer */
pub fn render_scene_threadpool(scene: &Scene) -> RgbaImage {
    /* Plutot que de modifier la memoire et de devoir la proteger pour communiquer entre differents threads,
     * On utilise les Channels qui permettent d'envoyer de maniere asynchrone depuis les threads enfants et de recuperer
     * toutes les communications depuis le thread parent */
    let (tx, rx) = mpsc::channel();

    /* Pour ne pas join a la main et pouvoir partager des reference immutables entre les threads, on declare un scope dans lequel
     * on spawn nos threads. Le scope s'assure que tout thread qui a ete spawn a l'interieur de ce scope est join avant la fin du scope. */
    let mut simplification_factor = 4;

    // while simplification_factor != 0 {
    let res_x = SCREEN_WIDTH / simplification_factor;
    let res_y = SCREEN_HEIGHT / simplification_factor;
    let nb_column_per_thread = res_x / MAX_THREADS;
    thread::scope(|s| {
        let mut last_x = 0;
        // 16 threads pour 16 core
        for id_thread in 0..MAX_THREADS {
            // On clone le transmitter pour pouvoir avoir plusieurs producers(les threads de calcul) pour un seul consumer(le main thread qui agrege)
            let cur_tx = tx.clone();
            let x;
            // On determine de quelles colonnes va s'occuper quel thread
            x = match id_thread < MAX_THREADS - 1 {
                true => res_x - (id_thread + 1) * nb_column_per_thread,
                false => 0,
            };
            let max_x = last_x;
            last_x = x;
            let res_x = res_x;
            let res_y = res_y;
            s.spawn(move || {
                let mut color_vec = vec![];
                let index = x;
                let camera = scene.camera();
                for x in index..max_x {
                    for y in 0..res_y {
                        let roll = get_angle_to(camera.fov(), x as f64, res_x as f64);
                        let pitch = get_angle_to(camera.vfov(), y as f64, res_y as f64);
                        // println!("x {} y {} roll {} pitch {}", x, y, roll, pitch);
                        let quat = Quaternion::from_euler_angles(pitch, roll, 0.);
                        let ray = Ray::new(
                            camera.pos().clone(),
                            camera.dir().clone().rotate(&quat).normalize(),
                            0,
                        );
                        color_vec.push(cast_ray(scene, &ray));
                    }
                }
                cur_tx.send((index, color_vec)).unwrap();
            });
        }
    });
    let result = build_image(rx, res_x as u32, res_y as u32);
    //     simplification_factor /= 2;
    // }
    result
}

fn build_image(rx: Receiver<(usize, Vec<Color>)>, res_x: u32, res_y: u32) -> RgbaImage {
    let mut img = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    let pixel_width = SCREEN_WIDTH as u32 / res_x;
    let pixel_height = SCREEN_HEIGHT as u32 / res_y;
    for (index, color_vec) in rx.try_iter() {
        let mut x_pixel = index as u32;
        let mut y_pixel = 0;
        for color in color_vec {
            let max_x = match x_pixel == res_x - 1 {
                false => (x_pixel + 1) * pixel_width,
                true => SCREEN_WIDTH as u32,
            };
            let max_y = match y_pixel == res_y - 1 {
                false => (y_pixel + 1) * pixel_height,
                true => SCREEN_HEIGHT as u32,
            };
            let rgba = color.to_rgba();
            for x in (x_pixel * pixel_width)..max_x {
                for y in (y_pixel * pixel_height)..max_y {
                    img.put_pixel(x, y, rgba);
                }
            }
            y_pixel += 1;
            if y_pixel == res_y {
                y_pixel = 0;
                x_pixel += 1;
            }
        }
    }
    img
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => apply_lighting(hit, scene, ray),
        None => Color::new(0., 0., 0.),
    }
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<(f64, &Element)> = None;
    for element in scene.elements().iter() {
        if let Some(t) = element.shape().intersect(ray) {
            if let Some((tmin, _)) = &closest {
                if &t[0] < tmin {
                    closest = Some((t[0], element));
                }
            } else {
                closest = Some((t[0], element))
            }
        }
    }
    match closest {
        None => None,
        Some((t, elem)) => Some(Hit::new(elem, t, ray.get_pos() + ray.get_dir() * t)),
    }
}
