use crate::{app::widgets::pixel_canvas::PixelCanvas, motor::{motor::Motor, scene::{BaseScene, Scene}}};

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

        self.base_scene.add_widget(
            Box::new(PixelCanvas::new("pixel canvas", 0))
        );
    }

    fn update(&mut self, motor: &mut Motor) {
        self.base_scene.update(motor);
    }

    fn render(&mut self, motor: &mut Motor, d: &mut crate::motor::motor::RenderTextureModeDrawHandle<'_>) {
        self.base_scene.render(motor, d);
    }
}