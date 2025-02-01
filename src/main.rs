use proj04::{app::scenes::game_of_life::GameOfLifeScene, motor::{motor::Motor, scene::Scene}};



fn main() {
    let mut motor = Motor::new(720, 480, "Game of Life");
    let mut first_scene = Box::new(GameOfLifeScene::new());
    motor.set_scene(first_scene);
    motor.run();
}
