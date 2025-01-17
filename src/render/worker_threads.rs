use std::{
    sync::{
        mpsc::Sender,
        Arc, RwLock,
    }, time::Instant
};
use crate::model::materials::color::Color;
use super::{
    common::{for_each_uncalculated_pixel, QueueContext, Tile}, raycasting::{get_lighting_from_ray, get_ray}
};

pub fn worker_thread (queue_context_list: Arc<RwLock<Vec<QueueContext>>>, cur_tx: Sender<(Tile, Vec<Color>)>) {
    let mut scene_index = 0;
    let mut tile_found: bool;
    let mut time_for_current_scene = 0.;
    let mut time_per_scene = 0.;
    loop {
        tile_found = false;
        while let Some(tile) = {
            get_next_tile(&mut scene_index, &queue_context_list)
        } {
            tile_found = true;
            let time = Instant::now();
            let mut colors = Vec::new();
            {
                let scene = tile.scene.read().unwrap();
                // Pour chaque pixel de cette tile qui n'a pas deja ete calcule par une taille de plus haut factor
                // Sur une tile 64x64 avec un factor de 64, il n'y aura qu'un ray de lance. Pour un factor de 32
                // 4 rays seront lancers (4 x 32x32 = 64x64)

                for_each_uncalculated_pixel(&tile, |x, y| {
                    // On calcule le ray et on le cast
                    let ray = get_ray(&scene, x, y);
                    colors.push(get_lighting_from_ray(&scene, &ray))
                });
            }
            cur_tx.send((tile, colors)).ok();

            // Pour pouvoir diviser le temps de compute entre toutes les scenes ouvertes equitablement, meme lorsqu'une scene prend plus de temps qu'une autre,
            // on doit mesurer le temps de render d'une tile pour chaque scene et render plusieurs tiles des scenes moins couteuse pour une seule des plus couteuses.
            // Ca nous permet de naviguer facilement dans une scene simple quand une scene complexe se render en arriere plan (mais avec un
            // temps de render plus eleve pour la scene complexe)
            if let Some(queue_context) = queue_context_list.write().unwrap().get_mut(scene_index) {
                queue_context.avg_time = (queue_context.avg_time * queue_context.tiles_done + time.elapsed().as_micros() as f64)/ (queue_context.tiles_done + 1.);
                queue_context.tiles_done += 1.;
                if queue_context.avg_time > time_per_scene {
                    time_per_scene = queue_context.avg_time;
                }
                time_for_current_scene += time.elapsed().as_micros() as f64;
                if time_for_current_scene > time_per_scene {
                    scene_index += 1;
                    time_for_current_scene = 0.;
                }
            }
        }
        // thread::sleep(Duration::from_millis(1));
        if !tile_found {
            scene_index += 1;
        }
        // Lorsqu'il n'y a plus de travail de disponible pour le moment, on ne surcharge pas la queue avec des reads constants.
    }
}

fn get_next_tile(scene_index: &mut usize, queue_context_list: &Arc<RwLock<Vec<QueueContext>>>) -> Option<Tile> {
    let mut queue_context_list = queue_context_list.write().unwrap();
    if queue_context_list.len() == 0 {
        return None;
    }
    if *scene_index >= queue_context_list.len() {
        *scene_index = 0;
    }
    let queue_context = &mut queue_context_list[*scene_index];
    if !queue_context.active {
        return None;
    }
    queue_context.queue.remove(0)
}