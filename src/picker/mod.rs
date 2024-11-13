use std::{
    fmt::format,
    fs, io,
    sync::{mpsc, Arc, RwLock},
    thread,
    time::Duration,
};

use image::{Rgba, RgbaImage};
use pixels::{Pixels, SurfaceTexture};
use rusttype::{Font, Scale};
use winit::{
    dpi::LogicalSize,
    event::{Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};

use crate::{
    display::display::display, model::{
        maths::{vec2::Vec2, vec3::Vec3},
        scene::Scene,
    }, parsing::get_scene, render::render_threads::start_render_threads, ui::{uisettings::UISettings, utils::{draw_utils::blend, style::Style}}, PICKER_LINE_HEIGHT, SCENE_FOLDER, SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32
};

pub fn get_files_in_folder(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    // Read the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and not a directory
        if path.is_file() {
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    files.push(filename_str.to_string());
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

fn get_line_position(i: usize) -> Vec2 {
    let x = PICKER_LINE_HEIGHT - 5.;
    let y = i as f64 * (PICKER_LINE_HEIGHT + 5.);

    Vec2::new(x, y)
}

fn draw_text(image: &mut RgbaImage, pos: &Vec2, text: String, format: &Style) {
    let x = *pos.x() as u32 + 8;
    let y = *pos.y() as u32 + 2;

    if y + PICKER_LINE_HEIGHT as u32 > SCREEN_HEIGHT_U32 {
        return;
    }

    // Load font
    let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
    let font = &Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // Set font size and color
    let scale = Scale::uniform(format.font_size());
    let color = format.font_color();

    // Draw text
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);

    for glyph in font.layout(&text, scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    *pixel = blend(color, pixel, v);
                }
            });
        }
    }
}

fn draw_text_background(image: &mut RgbaImage, pos: &Vec2) {
    let x: u32 = *pos.x() as u32;
    let y: u32 = *pos.y() as u32;
    let width: u32 = SCREEN_WIDTH_U32 / 2;
    let height: u32 = PICKER_LINE_HEIGHT as u32;

    if y + PICKER_LINE_HEIGHT as u32 > SCREEN_HEIGHT_U32 {
        return;
    }

    for i in x..(x + width) {
        for j in y..(y + height) {
            let pixel = image.get_pixel_mut(i, j);
            *pixel = Rgba([50, 50, 50, 255]);
        }
    }
}

fn get_hitbox(pos: &Vec2) -> (Vec2, Vec2) {
    let x = *pos.x();
    let y = *pos.y();
    let width = SCREEN_WIDTH_U32 / 2;
    let height = PICKER_LINE_HEIGHT;

    (Vec2::new(x, y), Vec2::new(x + width as f64, y + height))
}

fn draw_files_and_update_hitboxes(
    start: usize,
    files: &Vec<String>,
    pixels: &mut Pixels,
) -> (Vec<(Vec2, Vec2)>, RgbaImage) {
    let mut hitboxes: Vec<(Vec2, Vec2)> = Vec::new();
    let mut img = RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32);
    let settings = UISettings::default();
    let format = Style::default(&settings);

    for i in start..files.len() {
        let file = &files[i];
        let pos = get_line_position(i - start);
        hitboxes.push(get_hitbox(&pos));
        draw_text_background(&mut img, &pos);
        draw_text(&mut img, &pos, file.to_string(), &format);
    }

    draw_text(
        &mut img,
        &Vec2::new(860., 770.0),
        "Choose a scene".to_string(),
        &format,
    );
    draw_text(
        &mut img,
        &Vec2::new(860., 790.0),
        "Use the mouse wheel to scroll".to_string(),
        &format,
    );
    draw_text(
        &mut img,
        &Vec2::new(860., 810.0),
        "Press Enter to load the scene".to_string(),
        &format,
    );
    display(pixels, &mut img);
    return (hitboxes, img);
}

fn display_files(files: Vec<String>) -> String {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_title("Scene Picker")
        .build(&event_loop)
        .unwrap();

    // Set up pixels object
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    let (sender, receiver) = mpsc::channel();

    let mut hitboxes: Vec<(Vec2, Vec2)>;
    let mut img: RgbaImage;
    let mut file_clicked = "".to_string();

    // Draw the list of files and update the hitboxes
    let mut start = 0;
    (hitboxes, img) = draw_files_and_update_hitboxes(start, &files, &mut pixels);

    // Set up event manager
    let mut mouse_position = (0.0, 0.0);
    event_loop.run(move |event, window_target| {
        window_target.set_control_flow(ControlFlow::Wait);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // Close the window
                    sender.send("".to_string()).unwrap();
                    window_target.exit();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    // Update the mouse position
                    mouse_position = (position.x, position.y);
                }
                WindowEvent::MouseInput { state, .. } => {
                    // If the mouse is clicked
                    if state == winit::event::ElementState::Released {
                        // Get element clicked
                        let x = mouse_position.0 as u32;
                        let y = mouse_position.1 as u32;

                        file_clicked = get_file_name(&files, &hitboxes, x, y, start);
                        if file_clicked != "" {
                            // Display a preview of the scene
                            let path = format!("{}/{}", SCENE_FOLDER, file_clicked);
                            let scene = get_scene(&path);
                            render_preview(scene, &mut img, &mut pixels);
                        }
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    // check if the mouse wheel is scrolled up or down
                    let dir = get_scroll_direction(delta);

                    if dir == "up" {
                        // Scroll up
                        if start > 0 {
                            start -= 1;
                            (hitboxes, img) =
                                draw_files_and_update_hitboxes(start, &files, &mut pixels);
                        }
                    } else {
                        // Scroll down
                        if start < files.len() - 1 {
                            start += 1;
                            (hitboxes, img) =
                                draw_files_and_update_hitboxes(start, &files, &mut pixels);
                        }
                    }
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    // If the escape key is pressed
                    if event.logical_key == Key::Named(NamedKey::Escape) {
                        // Close the window
                        sender.send("".to_string()).unwrap();
                        window_target.exit();
                    } else if event.logical_key == Key::Named(NamedKey::Backspace) {
                        if file_clicked != "" {
                            sender.send(file_clicked.clone()).unwrap();
                            window_target.exit();
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }).unwrap();

    let path = receiver.recv().unwrap_or_else(|_| "".to_string());
    if path == "" {
        return "".to_string();
    }
    return format(format_args!("{}/{}", SCENE_FOLDER, &path));
}

fn get_scroll_direction(delta: MouseScrollDelta) -> String {
    match delta {
        MouseScrollDelta::LineDelta(_x, y) => {
            if y > 0.0 {
                return "up".to_string();
            } else {
                return "down".to_string();
            }
        }
        MouseScrollDelta::PixelDelta(pos) => {
            if pos.y > 0.0 {
                return "up".to_string();
            } else {
                return "down".to_string();
            }
        }
    }
}

fn get_file_name(
    files: &Vec<String>,
    hitboxes: &Vec<(Vec2, Vec2)>,
    x: u32,
    y: u32,
    start: usize,
) -> String {
    for (i, hitbox) in hitboxes.iter().enumerate() {
        let (min, max) = hitbox;
        if x >= *min.x() as u32
            && x <= *max.x() as u32
            && y >= *min.y() as u32
            && y <= *max.y() as u32
        {
            return files[i + start].to_string();
        }
    }

    return "".to_string();
}

pub fn pick_scene() -> String {
    // Get the list of scenes
    let files = get_files_in_folder(SCENE_FOLDER);

    // Display the list of scenes
    let path = display_files(files.unwrap());

    return path;
}

fn render_preview(mut scene: Scene, img: &mut RgbaImage, pixels: &mut Pixels) {
    // Setting up the render_threads and asking for the first image
    let camera = scene.camera_mut();
    camera.set_pos(camera.pos() + Vec3::new(0., 0., -10.));
    let scene = Arc::new(RwLock::new(scene));
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    let mut final_image = false;
    let mut preview_img: image::ImageBuffer<Rgba<u8>, Vec<u8>>;

    thread::sleep(Duration::from_millis(2));
    tb.send(false).unwrap();
    let (render_img, _) = ra.recv().unwrap();
    preview_img = render_img;
    let start = std::time::Instant::now();

    while start.elapsed().as_millis() < 1200 {
        if let Ok((render_img, final_img)) = ra.try_recv() {
            preview_img = render_img;
            final_image = final_img;
        }

        if !final_image {
            thread::sleep(Duration::from_millis(20));
            tb.send(false).unwrap();
        }
    }

    // let img_x_offset = 862;
    // let img_y_offset = 37;
    // let preview_x_offset = 450;
    // let preview_y_offset = 100;
    let img_x_offset = 1012;
    let img_y_offset = 37;
    let preview_x_offset = 600;
    let preview_y_offset = 250;

    for i in 0..400 {
        for j in 0..400 {
            let pixel = preview_img.get_pixel(i + preview_x_offset, j + preview_y_offset);
            img.put_pixel(i + img_x_offset, j + img_y_offset, *pixel);
        }
    }

    display(pixels, img);
}
