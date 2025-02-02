use crate::{app::widgets::pixel_canvas::PixelCanvas, motor::{button::Button, label::Label, motor::Motor, scene::{BaseScene, Scene}}};

pub struct GameOfLifeScene {
    base_scene: BaseScene,
}

impl GameOfLifeScene {
    pub fn new() -> GameOfLifeScene {
        GameOfLifeScene {
            base_scene: BaseScene::new(),
        }
    }
}

impl Scene for GameOfLifeScene {
    fn get_base_scene(&mut self) -> &mut BaseScene {
        &mut self.base_scene
    }

    fn init(&mut self, motor: &mut Motor) {
        self.base_scene.init(motor);

        const X: f32 = 540.0;
        self.base_scene.add_widget(
            Box::new(PixelCanvas::new("pixel-canvas", 0, 10.0, 10.0, 520.0, 460.0, 2.0))
        ).add_widget(
            Box::new(Label::new("utils", 1, X, 10.0, "UTILS", 20))
        ).add_widget(
            Box::new(Button::new("clear", 1, X, 40.0, "CLEAR")),
        ).add_widget(
            Box::new(Button::new("random", 1, X, 80.0, "RANDOM")),
        ).add_widget(
            Box::new(Label::new("speeds", 1, X, 130.0, "SPEEDS", 20))
        ).add_widget(
            Box::new(Button::new("pause", 1, X, 160.0, "PAUSE")),
        ).add_widget(
            Box::new(Button::new("slow", 1, X, 200.0, "SLOW")),
        ).add_widget(
            Box::new(Button::new("normal", 1, X, 240.0, "NORMAL")),
        ).add_widget(
            Box::new(Button::new("fast", 1, X, 280.0, "FAST")),
        ).add_widget(
            Box::new(Button::new("very-fast", 1, X, 320.0, "VERY FAST")),
        );

    }

    fn update(&mut self, motor: &mut Motor) {
        self.base_scene.update(motor);

        
            let pixel_canvas = self.base_scene.get_widget_by_name("pixel-canvas").unwrap().as_any_mut().downcast_mut::<PixelCanvas>().unwrap() as *mut PixelCanvas;
            let clear_button = self.base_scene.get_widget_by_name("clear").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
            let random_button = self.base_scene.get_widget_by_name("random").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
            let pause_button = self.base_scene.get_widget_by_name("pause").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
            let slow_button = self.base_scene.get_widget_by_name("slow").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
            let normal_button = self.base_scene.get_widget_by_name("normal").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
            let fast_button = self.base_scene.get_widget_by_name("fast").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
            let very_fast_button = self.base_scene.get_widget_by_name("very-fast").unwrap().as_any_mut().downcast_mut::<Button>().unwrap() as *mut Button;
        
        unsafe {
            (*random_button).clicked(motor).then(|| {
                (*pixel_canvas).random();
            });

            (*clear_button).clicked(motor).then(|| {
                (*pixel_canvas).clear();
            });

            (*pause_button).clicked(motor).then(|| {
                (*pixel_canvas).set_tick_speed(0);
            });

            (*slow_button).clicked(motor).then(|| {
                (*pixel_canvas).set_tick_speed(200);
            });

            (*normal_button).clicked(motor).then(|| {
                (*pixel_canvas).set_tick_speed(60);
            });

            (*fast_button).clicked(motor).then(|| {
                (*pixel_canvas).set_tick_speed(15);
            });

            (*very_fast_button).clicked(motor).then(|| {
                (*pixel_canvas).set_tick_speed(1);
            });
        }


    }

    fn render(&mut self, motor: &mut Motor, d: &mut crate::motor::motor::RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(motor, d);
    }
}