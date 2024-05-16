use crate::parsing::get_scene;

pub fn display_stereo_scene() {
    let mut scene_left = get_scene();
    let mut scene_right = get_scene();

    scene_left.camera_mut().move_left();
    scene_right.camera_mut().move_right();
    
}