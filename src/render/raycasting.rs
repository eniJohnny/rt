use std::{sync::mpsc::{self, Receiver}, thread};

use image::RgbaImage;

use crate::{model::{materials::Color, maths::{hit::Hit, ray::Ray, vec3::Vec3}, objects::camera::Camera, scene::Scene, Element}, MAX_THREADS, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::lighting::apply_lighting;



/**
 * Generation d'une matrice de ray selon les vecteurs camera.
 * Pour ca, on projete un ecran devant la camera a cam.pos + cam.dir, et passe les rays a travers tous les pixels de cet ecran.
 */
pub fn generate_rays(camera: &mut Camera) {
    // U et V sont les vecteurs unitaires de l'ecran projete.
    let u = Vec3::new(*camera.dir().z(), 0., - *camera.dir().x()).normalize();
    let v = - camera.dir().cross(&u).normalize();

    // Ajout de U et V a la camera
    camera.set_u(&u);
    camera.set_v(&v);

    // Tailles de l'ecran
    let width = (camera.fov()/2.).tan() * 2.;
    let height = width * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
    // Centre de l'ecran
    let center: Vec3 = camera.pos() + camera.dir();

    // Coin superieur gauche, et les distances pour atteindre a partir de lui les coin superieur droit et inferieur gauche
    let top_left = center +  &u * - width/2. - &v * height/2.;
    let left_to_right = &u * width;
    let top_to_bot = v * height;

    let mut result: Vec<Vec<Ray>> = vec![];
    for x in 0..SCREEN_WIDTH {
        let mut line: Vec<Ray> = vec![];
        for y in 0..SCREEN_HEIGHT {
            let pos = camera.pos().clone();
            // Du coup on fait le ratio entre sa position x, y et la taille en pixel de l'ecran, et on applique une
            // translation a partir du coing en haut a gauche pour nous donner le ray
            let dir = &top_left + &top_to_bot * (y as f64 / SCREEN_HEIGHT as f64) + &left_to_right * (x as f64 / SCREEN_WIDTH as f64) - &pos;
            let ray = Ray::new(pos, dir.normalize(), 0);    
            line.push(ray);
        }
        result.push(line);
    }
    camera.set_rays(result);
}

/* Divise la charge de travails entre 16 threads, et leur donne chacun un nombre de colonnes a calculer */
pub fn render_scene_threadpool(scene: &Scene) -> RgbaImage {
    // let time = Instant::now();
    /* Plutot que de modifier la memoire et de devoir la proteger pour communiquer entre differents threads,
     * On utilise les Channels qui permettent d'envoyer de maniere asynchrone depuis les threads enfants et de recuperer
     * toutes les communications depuis le thread parent */
    let (tx, rx) = mpsc::channel();
    let nb_column_per_thread = SCREEN_HEIGHT / MAX_THREADS;

    /* Pour ne pas join a la main et pouvoir partager des reference immutables entre les threads, on declare un scope dans lequel
     * on spawn nos threads. Le scope s'assure que tout thread qui a ete spawn a l'interieur de ce scope est join avant la fin du scope. */
    thread::scope(|s| {
        let rays = scene.camera().rays();
        let mut last_x = 0;
        // 16 threads pour 16 core
        for id_thread in 0..MAX_THREADS {
            // On clone le transmitter pour pouvoir avoir plusieurs producers(les threads de calcul) pour un seul consumer(le main thread qui agrege)
            let cur_tx = tx.clone();
            let mut x;
            // On determine de quelles colonnes va s'occuper quel thread
            x = match id_thread < MAX_THREADS - 1 {
                true => SCREEN_HEIGHT - (id_thread + 1) * nb_column_per_thread,
                false => 0
            };
            let columns = match id_thread == 0 {
                true => &rays[x..],
                false => &rays[x..last_x]
            };
            last_x = x;
             
            s.spawn(move || {
                let mut color_vec = vec![];
                let index = x;
                for column in columns {
                    for y in 0..SCREEN_HEIGHT {
                        color_vec.push(cast_ray(scene, &column[y]));
                    }
                    x += 1;
                }
                cur_tx.send((index, color_vec)).unwrap();
            });
        }
    });
    let result = build_image(rx);
    result
}

fn build_image(rx: Receiver<(usize, Vec<Color>)>) -> RgbaImage {
    let mut img = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    for (mut x, color_vec) in rx.try_iter() {
        let mut y = 0;
        for color in color_vec {
            img.put_pixel(x as u32, y as u32, color.to_rgba());
            y += 1;
            if y == SCREEN_HEIGHT {
                y = 0;
                x += 1;
            }
        }
    }
    img
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => apply_lighting(hit, scene, ray),
        None => Color::new(0., 0., 0.)
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
        Some((t, elem)) => Some(Hit::new(elem,t,ray.get_pos() + ray.get_dir() * t))
    }
}