use std::{collections::VecDeque, sync::{mpsc::{self, Receiver, Sender}, Arc, RwLock}, thread};

use image::RgbaImage;

use crate::{model::scene::Scene, MAX_THREADS};

use super::{render_thread::{render_thread, UIOrder}, worker_threads::worker_thread};



#[derive(Clone)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub factor: usize,
    pub base_factor: usize,
    pub scene: Arc<RwLock<Scene>>,
    pub render_id: usize,
    pub version: usize
}


pub struct QueueContext {
    pub queue: VecDeque<Tile>,
    pub render_id: usize,
    pub avg_time: f64,
    pub tiles_done: f64,
    pub active: bool
}

pub fn for_each_uncalculated_pixel<F>(tile: &Tile, mut f: F)
where
    F: FnMut(usize, usize),
{
    let mut offset_x = 0;
    let old_factor = tile.factor * 2;
    while offset_x < tile.width {
        let mut offset_y = 0;
        while offset_y < tile.height {
            let x = tile.x + offset_x;
            let y = tile.y + offset_y;
            if tile.factor == tile.base_factor {
                f(x, y);
            }
            f(x + tile.factor, y);
            f(x, y + tile.factor);
            f(x + tile.factor, y + tile.factor);
            offset_y += old_factor;
        }
        offset_x += old_factor;
    }
}



pub fn start_threads() -> (Receiver<(RgbaImage, bool)>, Sender<UIOrder>) {
    // Channel render_thread -> main_thread pour envoyer l'image du thread de render au main thread
    let (ta, ra) = mpsc::channel();
    // Channel main_thread -> render_thread pour recevoir les demandes d'images du main thread, avec ou sans changement de scene
    let (tb, rb) = mpsc::channel();
    // Channels worker_threads[] -> render_thread pour recevoir les tiles renderees par les worker thread qui travaillent constamment en fond
    // Communication de couleur finale des pixels
    let (tc, rc) = mpsc::channel();

    // La work_queue contient toutes les tiles en attente de render, dans toutes leurs versions de resolution(de 64x64:1 a 1x1:1)
    // Elle est protegee par un mutex car les worker threads vont constemment prendre du travail de cette queue, et le render thread
    // va en rajouter a chaque demande d'image


    let work_queues: Arc<RwLock<Vec<QueueContext>>> = Arc::new(RwLock::new(vec![]));
    // Generation du thread de render qui va lui-meme lancer les worker threads
    thread::spawn(move || {
        for _ in 0..MAX_THREADS {
            // Chaque worker_thread a son propre emetteur de channel, mais il n'existe qu'un seul receiver (le render_thread)
            let cur_tx = tc.clone();
            let work_queues_clone = Arc::clone(&work_queues);
            thread::spawn(move || {
                worker_thread(Arc::clone(&work_queues_clone), cur_tx);
            });
        }
        render_thread(rc, rb, ta, work_queues);
    });
    (ra, tb)
}