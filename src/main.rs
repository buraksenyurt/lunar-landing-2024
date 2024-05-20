use crate::constants::*;
use crate::game::Game;
use crate::viper::*;

mod constants;
mod entity;
mod game;
mod setup;
mod ui;
mod viper;

fn main() -> Result<(), String> {
    let game = Game::new();
    let screen = Screen::new(
        "Lunar Landing 2024".to_string(),
        Dimension::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
    );
    let mut engine = EngineBuilder::new()?
        .setup_screen(screen)?
        .add_game(Box::new(game))
        .change_fps(60)
        .build()?;
    engine.run()
}
