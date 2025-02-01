use std::any::Any;

use raylib::math::Vector2;

use super::motor::{Motor, RenderTextureModeDrawHandle};

pub trait Widget {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_base_widget(&self) -> &BaseWidget;
    fn get_base_widget_mut(&mut self) -> &mut BaseWidget;
    fn init(&mut self, motor: &mut Motor);
    fn update(&mut self, motor: &mut Motor);
    fn render(&mut self, motor: &mut Motor, d: &mut RenderTextureModeDrawHandle<'_> );
}

pub struct BaseWidget {
    name: String,
    layer: u16,

    pub position: Vector2,
}

impl BaseWidget {
    pub fn new(name: &str, layer: u16, x: f32, y: f32) -> BaseWidget {
        BaseWidget {
            name: name.to_string(),
            layer: layer,
            position: Vector2::new(x as f32, y as f32),
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_layer(&self) -> u16 {
        self.layer
    }
}
