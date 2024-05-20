use crate::viper::{AssetManager, GameObject, MainState};
use rand::prelude::ThreadRng;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

#[derive(Default)]
pub struct Game;

impl GameObject for Game {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        randomizer: &mut ThreadRng,
        delta_time: Duration,
    ) -> Result<MainState, String> {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(MainState::Exit),
                _ => {}
            }
        }
        Ok(MainState::Running)
    }

    fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_manager: &AssetManager,
    ) -> Result<(), String> {
        Ok(())
    }
}
