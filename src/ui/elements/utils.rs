use std::sync::{Arc, RwLock};

use crate::{model::scene::Scene, ui::{style::{Formattable, Style, StyleBuilder}, ui::UI, uibox::UIBox, uisettings::UISettings}};

use super::uielement::{Category, UIElement};

pub type FnSubmit = Box<dyn Fn(Option<&UIElement>, Value, &Arc<RwLock<Scene>>, &mut UI)>;
pub type FnApply = Box<dyn Fn(Option<&mut UIElement>, &Arc<RwLock<Scene>>, &mut UI)>;
pub type FnValidate = Box<dyn Fn(&Value) -> Result<(), &'static str>>;

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Float(f64),
    Unsigned(u32),
    Bool(bool),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Text(str) => str.clone(),
            Value::Bool(bool) => bool.to_string(),
            Value::Float(float) => float.to_string(),
            Value::Unsigned(unsigned) => unsigned.to_string(),
        }
    }
}

impl Formattable for Value {
    fn base_style(&self, settings: &UISettings) -> Style {
        Style::editing(settings)
    }
}

pub enum ElemType {
    Text,
    Stat(Box<dyn Fn(&Scene, &UI) -> String>),
    Property(Property),
    Category(Category),
    Button(Box<dyn Fn(&mut Scene, &mut UI)>),
    Row(Vec<UIElement>),
}

impl Formattable for ElemType {
    fn base_style(&self, settings: &UISettings) -> Style {
        match self {
            ElemType::Row(..) => Style::row(settings),
            ElemType::Button(..) => Style::button(settings),
            ElemType::Category(..) => Style::category(settings),
            ElemType::Property(..) => Style::property(settings),
            ElemType::Stat(..) => StyleBuilder::default(settings)
                .bg_color(None)
                .fill_width(true)
                .build(),
            ElemType::Text => {
                let mut format = Style::property(settings);
                format.bg_color = None;
                format
            }
        }
    }
}

pub struct Property {
    pub value: Value,
    pub initial_value: Value,
    pub editing_format: Style,
    pub fn_submit: FnSubmit,
    pub fn_validate: FnValidate,
}

impl Property {
    pub fn new(
        value: Value,
        fn_submit: FnSubmit,
        fn_validate: FnValidate,
        settings: &UISettings,
    ) -> Self {
        Self {
            editing_format: value.base_style(settings),
            initial_value: value.clone(),
            value,
            fn_submit,
            fn_validate,
        }
    }

    pub fn get_value_from_string(&self, val: String) -> Result<Value, String> {
        match self.value {
            Value::Bool(_) => return Err("Bool value edited ?".to_string()),
            Value::Float(_) => {
                let val = val.parse::<f64>();
                if val.is_err() {
                    return Err("The value must be a proper float".to_string());
                } else {
                    return Ok(Value::Float(val.unwrap()));
                }
            }
            Value::Text(_) => {
                return Ok(Value::Text(val));
            }
            Value::Unsigned(_) => {
                let val = val.parse::<u32>();
                if val.is_err() {
                    return Err("The value must be a proper unsigned integer".to_string());
                } else {
                    return Ok(Value::Unsigned(val.unwrap()));
                }
            }
        }
    }
}

