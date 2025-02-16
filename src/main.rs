use game_of_life::{app::scenes::game_of_life::GameOfLifeScene, motor::motor::Motor};



fn main() {
    let mut motor = Motor::new(720, 480, "Game of Life");
    let first_scene = Box::new(GameOfLifeScene::new());
    motor.set_scene(first_scene);
    motor.run();
}
