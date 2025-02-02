use raylib::prelude::*;

use super::{scene::Scene, texture_manager::TextureManager};

pub type RenderTextureModeDrawHandle<'x> = RaylibTextureMode<'x, RaylibDrawHandle<'x>>;

pub struct Motor {
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub texture_manager: TextureManager,
    pub canvas_size: (u32, u32),
    canvas: RenderTexture2D,

    running: bool,

    scene: Option<Box<dyn Scene>>,

    old_mouse_position: Vector2,
}

impl Motor {
    pub fn new(canvas_width: u32, canvas_height: u32, title: &str) -> Motor {
        let (mut rl, thread) = raylib::init()
            .size(canvas_width as i32, canvas_height as i32)
            .title(title)
            .resizable()
            .build();

        rl.set_target_fps(60);

        Motor{
            canvas: rl.load_render_texture(&thread, canvas_width, canvas_height ).unwrap(),
            texture_manager: TextureManager::new(
                &mut rl as *mut RaylibHandle,
                &thread as *const RaylibThread,
            ),
            rl: rl,
            thread: thread,
            
            running: false,
            scene: None,
            canvas_size: (canvas_width, canvas_height),

            old_mouse_position: Vector2::new(0.0, 0.0),
        }
    }
    
    pub fn run(&mut self) {
        self.running = true;

        

        while self.running {
            self.running = !self.rl.window_should_close();

            self.update();
            self.render();
        }
    }

    pub fn update(&mut self) {
        let aux = self as *mut Motor;
        self.get_scene().unwrap().update(unsafe { &mut *aux });

        self.old_mouse_position = self.get_mouse_position();
    }

    pub fn render(&mut self) {
        let win_width = self.rl.get_screen_width();
        let win_height = self.rl.get_screen_height();
        let canvas_width = self.canvas_size.0;
        let canvas_height = self.canvas_size.1;
        let mouse_position = self.get_mouse_position();

        let aux = self as *mut Motor;

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);
        {
            
            let mut t = d.begin_texture_mode(&self.thread, &mut self.canvas);
            t.clear_background(Color{r: 100,g: 100,b: 100,a: 255});
            
            self.scene.as_mut().unwrap().render( unsafe { &mut *aux }, &mut t);

            t.draw_circle_v(
                mouse_position,
                2.0,
                Color::WHITE,
            );
        }
        
        render_canvas(
            &mut d,
            &mut self.canvas,
            win_width,
            win_height,
            canvas_width as i32,
            canvas_height as i32,
        );
    }

    pub fn is_action_down(&self, action: &str) -> bool {
        match action {
            "up" => self.rl.is_key_down(KeyboardKey::KEY_W) || self.rl.is_key_down(KeyboardKey::KEY_UP),
            "down" => self.rl.is_key_down(KeyboardKey::KEY_S) || self.rl.is_key_down(KeyboardKey::KEY_DOWN),
            "right" => self.rl.is_key_down(KeyboardKey::KEY_D) || self.rl.is_key_down(KeyboardKey::KEY_RIGHT),
            "left" => self.rl.is_key_down(KeyboardKey::KEY_A) || self.rl.is_key_down(KeyboardKey::KEY_LEFT),
            
            "mouse_left_button" => self.rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT),
            "mouse_right_button" => self.rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT),

            "i" => self.rl.is_key_down(KeyboardKey::KEY_I),
            "k" => self.rl.is_key_down(KeyboardKey::KEY_K),
            _ => false,
        }
    }

    pub fn is_action_just_pressed(&self, action: &str) -> bool {
        match action {
            "mouse_left_button" => self.rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT),
            "mouse_wheel_up" => self.rl.get_mouse_wheel_move() > 0.0,
            "mouse_wheel_down" => self.rl.get_mouse_wheel_move() < 0.0,
            "start" => self.rl.is_key_pressed(KeyboardKey::KEY_ENTER),
            _ => false,
        }
    }

    pub fn get_mouse_motion(&self) -> Vector2 {
        let old_pos = self.old_mouse_position;
        let new_pos = self.get_mouse_position();

        new_pos - old_pos

    }

    pub fn get_mouse_position(&self) -> Vector2 {
        let raw_position = self.rl.get_mouse_position();

        let win_width = self.rl.get_screen_width();
        let win_height = self.rl.get_screen_height();

        let delta_x = win_width as f32 / self.canvas_size.0 as f32;
        let delta_y = win_height as f32 / self.canvas_size.1 as f32;

        let scale = if delta_x < delta_y { delta_x } else { delta_y };

        let scaled_width = self.canvas_size.0 as f32 * scale;
        let scaled_height = self.canvas_size.1 as f32 * scale;

        let diff_x = win_width as f32 - scaled_width;
        let diff_y = win_height as f32 - scaled_height;

        Vector2::new(
            (raw_position.x - diff_x / 2.0) / scale,
            (raw_position.y - diff_y / 2.0) / scale,
        )

    }

    pub fn get_scene(&mut self) -> Option<&mut Box<dyn Scene>> {
        self.scene.as_mut()
    }

    pub fn set_scene(&mut self, mut scene: Box<dyn Scene>) {
        scene.init(self);
        self.scene = Some(scene);
    }
}

fn render_canvas( d: &mut RaylibDrawHandle, canvas: &mut RenderTexture2D , win_width: i32, win_height: i32, canvas_width: i32, canvas_height: i32) {

    let delta_x = win_width as f32 / canvas_width as f32;
    let delta_y = win_height as f32 / canvas_height as f32;

    let scale = if delta_x < delta_y { delta_x } else { delta_y };

    let scaled_width = canvas_width as f32 * scale;
    let scaled_height = canvas_height as f32 * scale;

    let diff_x = win_width as f32 - scaled_width;
    let diff_y = win_height as f32 - scaled_height;

    d.draw_texture_pro(
        canvas.texture(),
        Rectangle::new(
            0.0,
            0.0,
            canvas_width as f32,
            -canvas_height as f32,
        ),
        Rectangle::new(diff_x / 2.0, diff_y / 2.0, scaled_width, scaled_height),
        Vector2::new(0.0, 0.0),
        0.0,
        Color::WHITE,
    );
}
