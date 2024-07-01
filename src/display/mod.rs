extern crate image;
extern crate pixels;
extern crate winit;

pub mod mainloop;
pub mod display;
pub mod events;

use crate::{
    model::scene::Scene, SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};
use mainloop::main_loop;
use image::RgbaImage;
use std::{
    ptr::copy_nonoverlapping,
    sync::{Arc, RwLock},
};

use crate::render::render_threads::start_render_threads;
use pixels::Pixels;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
