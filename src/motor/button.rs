use std::any::Any;

use super::{motor::{Motor, RenderTextureModeDrawHandle}, widget::{BaseWidget, Widget}};
use raylib::prelude::*;

const FONT_SIZE: i32 = 20;
const PADDING: f32 = 5.0;

pub struct Button {
    pub base_widget: BaseWidget,
    pub text: String,
    background_color: Color
}

impl Button {
    pub fn new(name: &str, layer: u16, x: f32, y: f32, text: &str) -> Button {
        Button {
            base_widget: BaseWidget::new(name, layer, x, y, 0.0, 0.0),
            text: text.to_string(),
            background_color: Color::WHITE
        }
    }

    pub fn clicked(&self, motor: &Motor) -> bool {
        self.base_widget.is_mouse_over(motor) && motor.is_action_just_pressed("mouse_left_button")
    }
}

impl Widget for Button {
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

    fn init(&mut self, motor: &mut Motor) {
        let text_width = motor.rl.measure_text( &self.text, FONT_SIZE );
        self.base_widget.size = Vector2::new( text_width as f32 + PADDING*2.0, FONT_SIZE as f32 + PADDING*2.0 );
    }
    fn update(&mut self, motor: &mut Motor) {

        if self.base_widget.is_mouse_over(motor) {
            self.background_color = Color::GRAY;
        } else {
            self.background_color = Color::WHITE;
        }
    }
    fn render(&mut self, _motor: &mut Motor, d: &mut RenderTextureModeDrawHandle<'_> ) {

        d.draw_rectangle(
            self.base_widget.position.x as i32, 
            self.base_widget.position.y as i32, 
            self.base_widget.size.x as i32, 
            self.base_widget.size.y as i32, 
            self.background_color,
        );
        
        d.draw_text(
            &self.text, 
            self.base_widget.position.x as i32 + PADDING as i32, 
            self.base_widget.position.y as i32 + PADDING as i32, 
            FONT_SIZE as i32, 
            Color::BLACK,
        );
    }
}