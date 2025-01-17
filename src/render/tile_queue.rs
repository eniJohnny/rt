use std::{sync::{Arc, RwLock}, cmp::min};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

use super::{common::{QueueContext, Tile}, render_thread::SceneRender};


pub fn fill_work_queue(render: &mut SceneRender, scene_queue_list: &Arc<RwLock<Vec<QueueContext>>>,simplification_factor: usize) {
    {
        for queue_context in scene_queue_list.write().unwrap().iter_mut() {
            if render.render_id == queue_context.render_id {
                queue_context.queue.clear();
                break;
            }
        }
    }

    render.low_res_to_do = generate_tiles_for(
        render,
        &scene_queue_list,
        TILE_SIZE,
        simplification_factor,

    );
    render.max_res_to_do = render.low_res_to_do;
}

fn generate_tiles_for(
    render: &mut SceneRender,
    queue: &Arc<RwLock<Vec<QueueContext>>>,
    size: usize,
    base_factor: usize
) -> u32 {
    let mut cpt = 0;
    let mut factor = base_factor;
    // Pour chaque resolution possible on genere les demandes de render de tiles, en commencant
    // par la resolution la plus basse pour qu'elle soit render en premier.
    // On a beau tout envoyer d'un coup dans la queue, les worker_threads sont eveilles pendant ce temps
    // et prennent les ordres au fur et a mesure qu'ils sont ajoutes ce qui evite une perte de performance.
    let mut queue_list = queue.write().unwrap();
    let mut work_queue = None;
    for queue in queue_list.iter_mut() {
        if queue.render_id == render.render_id {
            work_queue = Some(queue);
            break;
        }
    }
    if let Some(wq) = work_queue {
        while factor != 0 {
            let mut x = 0;
            while x < SCREEN_WIDTH {
                let mut y = 0;
                while y < SCREEN_HEIGHT {
                    let width = min(SCREEN_WIDTH - x, size);
                    let height = min(SCREEN_HEIGHT - y, size);
                    if factor == 1 {
                        cpt += 1;
                    }
                    wq.queue.push_back(Tile {
                        x,
                        y,
                        width,
                        height,
                        base_factor,
                        factor,
                        render_id: render.render_id,
                        scene: render.scene.clone(),
                        version: render.version
                    });
                    y += size;
                }
                x += size;
            }
            factor /= 2;
        }
    }
    cpt
}