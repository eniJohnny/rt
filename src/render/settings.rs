use crate::{
    display::{anaglyph::Coloring, filters::ColorFilter}, model::{
        materials::{color::Color, texture::{Texture, TextureType}},
        maths::vec3::Vec3, objects::lights::parallel_light::ParallelLight,
    }, ui::{
        prefabs::{texture_ui::get_texture_ui, vector_ui::get_vector_ui}, uielement::{Category, UIElement}, uisettings::UISettings, utils::{
            misc::{ElemType, Property, Value}, ui_utils::UIContext, Displayable
        }
    }, ANAGLYPH_OFFSET_X, ANAGLYPH_OFFSET_Y, ANTIALIASING, DISPLACEMENT, MAX_DEPTH, MAX_ITERATIONS, PLANE_DISPLACED_DISTANCE, PLANE_DISPLACEMENT_STEP, SKYBOX_TEXTURE, SPHERE_DISPLACED_DISTANCE, SPHERE_DISPLACEMENT_STEP, VIEW_MODE
};

#[derive(Debug, Clone)]
pub enum ViewMode {
    Simple(Color, ParallelLight),
    Phong,
    Norm,
    HighDef,
    BVH,
}

#[derive(Debug)]
pub struct Settings {
    pub reflections: bool,
    pub indirect: bool,
    pub iterations: usize,
    pub skybox_texture: String,
    pub depth: usize,
    pub anti_alisaing: f64,
    pub displacement: bool,
    pub plane_displaced_distance: f64,
    pub plane_displacement_step: f64,
    pub sphere_displaced_distance: f64,
    pub sphere_displacement_step: f64,
    pub view_mode: ViewMode,
    pub bvh_full_traversal: bool,
    pub filter: ColorFilter
}

impl Settings {
    pub fn default() -> Self {
        let view_mode = match VIEW_MODE {
            "Norm" => ViewMode::Norm,
            "HighDef" => ViewMode::HighDef,
            "BVH" => ViewMode::BVH,
            "Simple" | _ => ViewMode::Simple(
                Color::new(0.2, 0.2, 0.2),
                ParallelLight::new(Vec3::new(0.5, -0.5, 0.5), 1., Color::new(1., 1., 1.)),
            )
        };

        Self {
            view_mode: view_mode,
            reflections: true,
            indirect: true,
            iterations: MAX_ITERATIONS,
            displacement: DISPLACEMENT,
            skybox_texture: SKYBOX_TEXTURE.to_string(),
            plane_displaced_distance: PLANE_DISPLACED_DISTANCE,
            plane_displacement_step: PLANE_DISPLACEMENT_STEP,
            sphere_displaced_distance: SPHERE_DISPLACED_DISTANCE,
            sphere_displacement_step: SPHERE_DISPLACEMENT_STEP,
            depth: MAX_DEPTH,
            anti_alisaing: ANTIALIASING,
            bvh_full_traversal: false,
            filter: ColorFilter::None
        }
    }
}

impl Displayable for Settings {
    fn get_fields(&self, name: &str, _context: &UIContext, settings: &UISettings) -> Vec<UIElement> {
        let mut category = Category::default();
        category.elems.push(UIElement::new(
            "Iterations",
            "iterations",
            ElemType::Property(Property::new(
                Value::Unsigned(self.iterations as u32),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Unsigned(value) = value {
                        scene.write().unwrap().settings_mut().iterations = value as usize;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_, _, _| Ok(())),
                settings,
            )),
            settings,
        ));
		let mut ambient_category = UIElement::new("Ambient light", "ambient", ElemType::Category(Category::default()), settings);
		ambient_category.add_element(get_vector_ui(Vec3::new(0., 0., 0.), "Color", "ambient.color", settings, 
		Box::new(move |_, value, scene, _| {}), 
		Box::new(move |_, value, scene, _| {}), 
		Box::new(move |_, value, scene, _| {}), true, None, None));
		category.elems.push(ambient_category);
		category.elems.push(get_texture_ui("Skybox", &Texture::Value(Vec3::new(0.2, 0.2, 0.2), TextureType::Color), Box::new(
            |value, scene| {
                let mut scene = scene.write().unwrap();
                if let Texture::Texture(path, _) = &value {
                    scene.load_texture(&path);
                }
                scene.set_skybox(value);
                scene.set_dirty(true);
            },
        ), settings, true, false, None, None, None));
        category.elems.push(UIElement::new(
            "Ray depth",
            "depth",
            ElemType::Property(Property::new(
                Value::Unsigned(self.depth as u32),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Unsigned(value) = value {
                        scene.write().unwrap().settings_mut().depth = value as usize;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_, _, _| Ok(())),
                settings,
            )),
            settings,
        ));
        category.elems.push(UIElement::new(
            "Displacement",
            "displacement",
            ElemType::Property(Property::new(
                Value::Bool(self.displacement),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Bool(value) = value {
                        scene.write().unwrap().settings_mut().displacement = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_, _, _| Ok(())),
                settings,
            )),
            settings,
        ));
        
        let plane_displacement_vec = vec![UIElement::new(
            "Plane displaced distance",
            "plane_displaced_factor",
            ElemType::Property(Property::new(
                Value::Float(self.plane_displaced_distance),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Float(value) = value {
                        scene.write().unwrap().settings_mut().plane_displaced_distance = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|value, _, _| {
                    if let Value::Float(value) = value {
                        if *value < 0. {
                            return Err(String::from("This value must be positive"))
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        ),
        UIElement::new(
            "Plane displacement step",
            "plane_displacement_step",
            ElemType::Property(Property::new(
                Value::Float(self.plane_displacement_step),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Float(value) = value {
                        scene.write().unwrap().settings_mut().plane_displacement_step = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|value, _, _| {
                    if let Value::Float(value) = value {
                        if *value < 0. {
                            return Err(String::from("This value must be positive"))
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        )];
        category.elems.push(UIElement::new("", "row_plane_displacement", ElemType::Row(plane_displacement_vec), settings));
        let sphere_displacement_vec = vec![UIElement::new(
            "Sphere displaced distance",
            "sphere_displaced_factor",
            ElemType::Property(Property::new(
                Value::Float(self.sphere_displaced_distance),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Float(value) = value {
                        scene.write().unwrap().settings_mut().sphere_displaced_distance = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|value, _, _| {
                    if let Value::Float(value) = value {
                        if *value < 0. {
                            return Err(String::from("This value must be positive"))
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        ),
        UIElement::new(
            "Sphere displacement step",
            "sphere_displacement_step",
            ElemType::Property(Property::new(
                Value::Float(self.sphere_displacement_step),
                Box::new(|_, value: Value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Float(value) = value {
                        scene.write().unwrap().settings_mut().sphere_displacement_step = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|value, _, _| {
                    if let Value::Float(value) = value {
                        if *value < 0. {
                            return Err(String::from("This value must be positive"))
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        )];
        category.elems.push(UIElement::new("", "row_sphere_displacement", ElemType::Row(sphere_displacement_vec), settings));
        let chk_reflect = UIElement::new(
            "Reflections",
            "chk_reflect",
            ElemType::Property(Property::new(
                Value::Bool(self.reflections),
                Box::new(|_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Bool(value) = value {
                        scene.write().unwrap().settings_mut().reflections = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_, _, _| Ok(())),
                settings,
            )),
            settings,
        );
        let chk_indirect = UIElement::new(
            "Indirect light",
            "chk_indirect",
            ElemType::Property(Property::new(
                Value::Bool(self.indirect),
                Box::new(|_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    if let Value::Bool(value) = value {
                        scene.write().unwrap().settings_mut().indirect = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_, _, _| Ok(())),
                settings,
            )),
            settings,
        );
        let vec = vec![chk_indirect, chk_reflect];
        let row = UIElement::new("", "row_indirect_reflection", ElemType::Row(vec), settings);
        category.elems.push(row);

        let mut view_mode_radio = UIElement::new("", "viewmode", ElemType::Row(vec![]), settings);
        let mut simple = UIElement::new(
            "Simple",
            "simple",
            ElemType::Button(Some(Box::new(|_, context, _| {
                if let Some(scene) = context.get_active_scene() {
                    scene.write().unwrap().settings_mut().view_mode = ViewMode::Simple(
                        Color::new(0.2, 0.2, 0.2),
                        ParallelLight::new(Vec3::new(0.5, -0.5, 0.5), 1., Color::new(1., 1., 1.)),
                    );
                    scene.write().unwrap().set_dirty(true);
                }
            }))),
            settings,
        );
        let mut gi = UIElement::new(
            "Global",
            "gi",
            ElemType::Button(Some(Box::new(|_, context, _| {
                if let Some(scene) = context.get_active_scene() {
                    scene.write().unwrap().settings_mut().view_mode = ViewMode::HighDef;
                    scene.write().unwrap().set_dirty(true);
                }
            }))),
            settings,
        );
        let mut norm = UIElement::new(
            "Normals",
            "norm",
            ElemType::Button(Some(Box::new(|_, context, _ui| {
                if let Some(scene) = context.get_active_scene() {
                    scene.write().unwrap().settings_mut().view_mode = ViewMode::Norm;
                    scene.write().unwrap().set_dirty(true);
                }
            }))),
            settings,
        );
        let mut phong = UIElement::new(
            "Phong",
            "phong",
            ElemType::Button(Some(Box::new(|_, context, _ui| {
                if let Some(scene) = context.get_active_scene() {
                    scene.write().unwrap().settings_mut().view_mode = ViewMode::Phong;
                    scene.write().unwrap().set_dirty(true);
                }
            }))),
            settings,
        );

        norm.style_mut().fill_width = true;
        gi.style_mut().fill_width = true;
        simple.style_mut().fill_width = true;
        phong.style_mut().fill_width = true;
        view_mode_radio.add_element(simple);
        view_mode_radio.add_element(phong);
        view_mode_radio.add_element(norm);
        view_mode_radio.add_element(gi);

        let mut filter_radio = UIElement::new("", "filter", ElemType::Row(vec![]), settings);
        let mut no_filter = UIElement::new(
            "None",
            "none",
            ElemType::Button(Some(Box::new(|_, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                scene.write().unwrap().settings_mut().filter = ColorFilter::None;
            }))),
            settings,
        );
        let mut sepia_filter = UIElement::new(
            "Sepia",
            "sepia",
            ElemType::Button(Some(Box::new(|_, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    scene.write().unwrap().settings_mut().filter = ColorFilter::Sepia;
            }))),
            settings,
        );
        let mut grayscale_filter = UIElement::new(
            "GrayScale",
            "grayscale",
            ElemType::Button(Some(Box::new(|_, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    scene.write().unwrap().settings_mut().filter = ColorFilter::GrayScale;
            }))),
            settings,
        );
        let mut cartoon_filter = UIElement::new(
            "Cartoon",
            "cartoon",
            ElemType::Button(Some(Box::new(|_, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                scene.write().unwrap().settings_mut().filter = ColorFilter::Cartoon;
            }))),
            settings,
        );
        let mut anaglyph_filter = UIElement::new(
            "Anaglyph",
            "anaglyph",
            ElemType::Button(Some(Box::new(|_, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                scene.write().unwrap().settings_mut().filter = ColorFilter::Anaglyph(ANAGLYPH_OFFSET_X, ANAGLYPH_OFFSET_Y, Coloring::RedGreen);
            }))),
            settings,
        );
        sepia_filter.style_mut().fill_width = true;
        cartoon_filter.style_mut().fill_width = true;
        grayscale_filter.style_mut().fill_width = true;
        anaglyph_filter.style_mut().fill_width = true;
        no_filter.style_mut().fill_width = true;
        filter_radio.add_element(no_filter);
        filter_radio.add_element(sepia_filter);
        filter_radio.add_element(cartoon_filter);
        filter_radio.add_element(grayscale_filter);
        filter_radio.add_element(anaglyph_filter);
        let mut category = UIElement::new(
            name,
            "settings",
            ElemType::Category(category),
            settings,
        );

        category.add_element(view_mode_radio);
        category.add_element(filter_radio);

        vec![category]
    }
}
