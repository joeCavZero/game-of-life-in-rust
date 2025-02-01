use std::any::Any;

use raylib::prelude::*;

use crate::motor::{motor::{Motor, RenderTextureModeDrawHandle}, widget::{BaseWidget, Widget}};

const PIXEL_SIZE: i32 = 10;
const PIXEL_CANVAS_LENGHT: usize = 64;
const PIXEL_CANVAS_SIZE: usize = PIXEL_CANVAS_LENGHT * PIXEL_CANVAS_LENGHT;
pub struct PixelCanvas {
    base_widget: BaseWidget,
    pixels: [u8; PIXEL_CANVAS_LENGHT * PIXEL_CANVAS_LENGHT],
    tick_delay: u16,

    inside_position: Vector2,

    zoom: f32,
}

impl PixelCanvas {
    pub fn new(name: &str, layer: u16, x: f32, y: f32, width: f32, height: f32, zoom: f32) -> PixelCanvas {
        PixelCanvas {
            base_widget: BaseWidget::new(name, layer, x ,y , width, height),
            pixels: [0; PIXEL_CANVAS_LENGHT * PIXEL_CANVAS_LENGHT],
            tick_delay: 0,
            inside_position: Vector2::new(0.0, 0.0),
            zoom: zoom,
        }
    }

    pub fn tick(&mut self) {
        let mut updated_pixels = [0; PIXEL_CANVAS_SIZE];

        for i in 0..self.pixels.len() as u64 {
            let mut neighbors = 0;
            let x = i % PIXEL_CANVAS_LENGHT as u64;
            let y = i / PIXEL_CANVAS_LENGHT as u64;

            for delta_x in -1i64..=1 {
                for delta_y in -1i64..=1 {
                    if delta_x == 0 && delta_y == 0 {
                        continue; // skip if it is the center pixel/itself
                    }

                    if self.get_pixel(x as i64 + delta_x, y as i64 + delta_y) > 0 {
                        neighbors += 1;
                    }
                }
            }

            if neighbors == 3 {
                update_pixel(&mut updated_pixels, x as i64, y as i64, 1);
            } else if neighbors == 2 && self.get_pixel(x as i64, y as i64) > 0 {
                update_pixel(&mut updated_pixels, x as i64, y as i64, 1);
            } else {
                update_pixel(&mut updated_pixels, x as i64, y as i64, 0);
            }
        }

        self.pixels = updated_pixels;
    }

    pub fn get_pixel(&self, x: i64, y: i64) -> u8 {

        match self.pixels.get((y * PIXEL_CANVAS_LENGHT as i64 + x) as usize) {
            Some(pixel) => *pixel,
            None => 0,
        }
    }


    pub fn get_relative_mouse_position(&self, motor: &Motor) -> Vector2 {
        let raw_position = motor.get_mouse_position();
        let position = self.base_widget.position;
        let inside_position = self.inside_position;

        Vector2::new(
            (raw_position.x - position.x - inside_position.x) / self.zoom,
            (raw_position.y - position.y - inside_position.y) / self.zoom,
        )
    }
}

fn update_pixel(arr: &mut [u8; PIXEL_CANVAS_SIZE], x: i64, y: i64, value: u8) {
    if x < 0 || y < 0 || x >= PIXEL_CANVAS_LENGHT as i64 || y >= PIXEL_CANVAS_LENGHT as i64 {
        return;
    }

    arr[(y * PIXEL_CANVAS_LENGHT as i64 + x) as usize] = value;
}

impl Widget for PixelCanvas {
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

    


    fn init(&mut self, motor: &mut Motor) {}
    fn update(&mut self, motor: &mut Motor) {
        self.tick_delay += 1;
        if self.tick_delay >= 120 {
            self.tick();
            self.tick_delay = 0;
        }

        if motor.is_action_just_pressed("mouse_left_click") {
            let mouse_pos = self.get_relative_mouse_position(motor);
            let x = (mouse_pos.x / 10.0) as i64;
            let y = (mouse_pos.y / 10.0) as i64;

            update_pixel(&mut self.pixels, x, y, 1);
        }

        if motor.is_action_down("i") {
            self.zoom += 0.1;
        } else if motor.is_action_down("k") {
            self.zoom -= 0.1;
        }

        if self.zoom < 0.1 {
            self.zoom = 0.1;
        }

        if motor.is_action_down("up") {
            self.inside_position.y -= 10.0;
        } else if motor.is_action_down("down") {
            self.inside_position.y += 10.0;
        }

        if motor.is_action_down("left") {
            self.inside_position.x -= 10.0;
        } else if motor.is_action_down("right") {
            self.inside_position.x += 10.0;
        }
    }
    fn render(&mut self, _motor: &mut Motor, d: &mut RenderTextureModeDrawHandle<'_> ) {
        d.draw_rectangle(
            self.base_widget.position.x as i32, 
            self.base_widget.position.y as i32, 
            self.base_widget.size.x as i32,
            self.base_widget.size.y as i32, 
            Color::BLACK,
        );

        let inside_x = (self.base_widget.position.x + self.inside_position.x).clamp(self.base_widget.position.x, self.base_widget.position.x + self.base_widget.size.x);
        let inside_y = (self.base_widget.position.y + self.inside_position.y).clamp(self.base_widget.position.y, self.base_widget.position.y + self.base_widget.size.y);
        let inside_width = (self.base_widget.size.x * self.zoom).min(self.base_widget.size.x - (inside_x - self.base_widget.position.x));
        let inside_height = (self.base_widget.size.y * self.zoom).min(self.base_widget.size.y - (inside_y - self.base_widget.position.y));

        d.draw_rectangle(
            inside_x as i32,
            inside_y as i32,
            inside_width as i32,
            inside_height as i32,
            Color::RED,
        );

        let zoom: f32 = self.zoom;

        for i in 0..self.pixels.len() {
            let x = i % PIXEL_CANVAS_LENGHT;
            let y = i / PIXEL_CANVAS_LENGHT;

            if self.pixels[i] > 0 {
                let pixel_x = (self.base_widget.position.x + self.inside_position.x + x as f32 * PIXEL_SIZE as f32 * zoom) as i32;
                let pixel_y = (self.base_widget.position.y + self.inside_position.y + y as f32 * PIXEL_SIZE as f32 * zoom) as i32;
                let pixel_width = PIXEL_SIZE as f32 * zoom;
                let pixel_height = PIXEL_SIZE as f32 * zoom;

                // the pixel drawing must be inside the the second draw above

                let inside_pixel_x = (pixel_x as f32).clamp(inside_x, inside_x + inside_width);
                let inside_pixel_y = (pixel_y as f32).clamp(inside_y, inside_y + inside_height);
                let inside_pixel_width = (pixel_width).min(inside_x + inside_width - inside_pixel_x);
                let inside_pixel_height = (pixel_height).min(inside_y + inside_height - inside_pixel_y);
                d.draw_rectangle(
                    inside_pixel_x as i32,
                    inside_pixel_y as i32,
                    inside_pixel_width as i32,
                    inside_pixel_height as i32,
                    Color::WHITE,
                );
                
            }
        }
    }
}