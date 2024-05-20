use crate::viper::asset_manager::AssetManager;
use rand::rngs::ThreadRng;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub trait GameObject {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        randomizer: &mut ThreadRng,
        delta_time: Duration,
    ) -> Result<MainState, String>;

    fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &AssetManager,
    ) -> Result<(), String>;
}

pub enum MainState {
    Exit,
    Running,
}
