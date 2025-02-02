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
            Box::new(PixelCanvas::new("pixel canvas", 0, 10.0, 10.0, 520.0, 460.0, 2.0))
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
            Box::new(Button::new("very_fast", 1, X, 320.0, "VERY FAST")),
        );

    }

    fn update(&mut self, motor: &mut Motor) {
        self.base_scene.update(motor);
    }

    fn render(&mut self, motor: &mut Motor, d: &mut crate::motor::motor::RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(motor, d);
    }
}