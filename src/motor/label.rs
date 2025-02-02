use std::any::Any;

use super::{motor::{Motor, RenderTextureModeDrawHandle}, widget::{BaseWidget, Widget}};
use raylib::prelude::*;

pub struct Label {
    pub base_widget: BaseWidget,
    pub text: String,
    font_size: i32,
}

impl Label {
    pub fn new(name: &str, layer: u16, x: f32, y: f32, text: &str, font_size: i32) -> Label {
        Label {
            base_widget: BaseWidget::new(name, layer, x, y, 0.0, 0.0),
            text: text.to_string(),
            font_size: font_size,
        }
    }
}

impl Widget for Label {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_base_widget(&self) -> &BaseWidget {
        &self.base_widget
    }

    fn get_base_widget_mut(&mut self) -> &mut BaseWidget {
        &mut self.base_widget
    }

    fn init(&mut self, _motor: &mut Motor) {}
    fn update(&mut self, _motor: &mut Motor) {}
    fn render(&mut self, _motor: &mut Motor, d: &mut RenderTextureModeDrawHandle<'_> ) {

        d.draw_text(
            &self.text, 
            self.base_widget.position.x as i32, 
            self.base_widget.position.y as i32, 
            self.font_size, 
            Color::WHITE,
        );
    }
}