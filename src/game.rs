use crate::constants::*;
use crate::entity::*;
use crate::setup::*;
use crate::ui::*;
use crate::viper::*;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;
use sdl2::EventPump;
use std::cmp;
use std::collections::HashMap;
use std::time::Duration;

pub struct Game {
    pub shuttle: Shuttle,
    pub mountain_points: Vec<Point>,
    pub landing_platforms: Vec<LandingPlatform>,
    pub meteors: Vec<Meteor>,
    pub stars: Vec<Star>,
    pub state: GameState,
    pub delta_second: Duration,
    pub distances: Vec<f32>,
    pub hud: Hud,
    pub play_commands: HashMap<Keycode, Box<dyn DirectionCommand>>,
    pub menu_commands: HashMap<Keycode, Box<dyn MenuCommand>>,
}

impl Game {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let ground_level = SCREEN_HEIGHT - 100;

        let mut mountain_points = Vec::new();
        let mut x = 0;
        while x < SCREEN_WIDTH {
            let peak_height = rng.gen_range(ground_level - 150..ground_level);
            mountain_points.push(Point::new(x, peak_height));
            x += rng.gen_range(80..120);
        }
        mountain_points.push(Point::new(
            SCREEN_WIDTH,
            rng.gen_range(ground_level - 150..ground_level),
        ));

        let mut platforms = Vec::new();
        let mut used_x_positions = Vec::new();
        while platforms.len() < 2 {
            let mut landing_start = rng.gen_range(0..SCREEN_WIDTH - 50);
            while used_x_positions
                .iter()
                .any(|&pos| (landing_start..landing_start + 50).contains(&pos))
            {
                landing_start = rng.gen_range(0..SCREEN_WIDTH - 50);
            }
            let landing_end = cmp::min(landing_start + 65, SCREEN_WIDTH);
            used_x_positions.extend(landing_start..landing_end);

            let platform_height = rng.gen_range(ground_level - 150..ground_level - 100);
            let l_leg = Math::find_ground_height(&mountain_points, landing_start);
            let r_leg = Math::find_ground_height(&mountain_points, landing_end);

            platforms.push(LandingPlatform::new(
                Point::new(landing_start, platform_height),
                Point::new(landing_end, platform_height),
                Point::new(landing_start, l_leg),
                Point::new(landing_end, r_leg),
            ));
        }
        let mut meteors = Vec::new();
        for _ in 1..=MAX_METEORS_COUNT {
            let m = Self::spawn_random_meteor();
            meteors.push(m);
        }

        let mut stars = Vec::new();
        for _ in 0..MAX_STARS_COUNT {
            let x = rng.gen_range(0..SCREEN_WIDTH);
            let y = rng.gen_range(0..SCREEN_HEIGHT / 2);

            let star = Star::new(
                (Point::new(x - 5, y), Point::new(x + 5, y)),
                (Point::new(x, y - 5), Point::new(x, y + 5)),
                (Point::new(x - 3, y - 3), Point::new(x + 3, y + 3)),
                (Point::new(x - 3, y + 3), Point::new(x + 3, y - 3)),
            );

            stars.push(star);
        }

        let (play_commands, menu_commands) = setup_commands().unwrap();

        Self {
            shuttle: Shuttle::new(),
            mountain_points,
            landing_platforms: platforms,
            meteors,
            stars,
            state: GameState::Menu,
            delta_second: Duration::default(),
            distances: vec![],
            hud: Hud::new(),
            play_commands,
            menu_commands,
        }
    }

    fn calc_distances(&mut self) {
        self.distances.clear();

        let shuttle_x = self.shuttle.position.x as f32
            + self.shuttle.velocity.x
            + (SHUTTLE_HEAD_WIDTH / 2) as f32;
        let shuttle_y = self.shuttle.position.y as f32
            + self.shuttle.velocity.y
            + SHUTTLE_HEAD_WIDTH as f32 * 2.;

        for lp in self.landing_platforms.iter() {
            let dist_x = shuttle_x - (lp.p1.x + (lp.p2.x - lp.p1.x) / 2) as f32;
            let dist_y = shuttle_y - lp.p1.y as f32;
            let dist = (dist_x.powi(2) + dist_y.powi(2)).sqrt();
            self.distances.push(dist);
        }
    }

    fn check_shuttle(&mut self) -> Option<Option<GameState>> {
        if self.shuttle.fuel_level <= 0 {
            self.state = GameState::OutOfFuel;
            return Some(Some(GameState::OutOfFuel));
        }
        if !self.shuttle.is_landed(self) {
            self.shuttle
                .toss_randomly(Vector { x: 40., y: 80. }, self.delta_second.as_secs_f32());
            self.shuttle.velocity.y += 2.5 * self.delta_second.as_secs_f32();
            self.shuttle.fuel_level -= 1;
            Some(None)
        } else {
            self.state = GameState::JobsDone;
            Some(Some(GameState::JobsDone))
        }
    }

    fn move_meteors(&mut self, delta_time: f32) {
        let mut rng = thread_rng();
        for m in self.meteors.iter_mut() {
            m.rot_angle += 25. * delta_time as f64;
            m.velocity.y += rng.gen_range(10.0..15.) * delta_time;
            match m.kind {
                MeteorType::LeftBottomCorner => {
                    m.velocity.x -= rng.gen_range(10.0..15.) * delta_time;
                }
                MeteorType::RightBottomCorner => {
                    m.velocity.x += rng.gen_range(10.0..15.) * delta_time;
                }
                MeteorType::Vertical => {
                    m.velocity.y += 10. * delta_time;
                }
            }
            m.mark_range();
        }
    }

    fn check_out_of_ranges(&mut self) {
        self.meteors.retain(|m| m.in_range);
    }

    fn spawn_random_meteor() -> Meteor {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..SCREEN_WIDTH - SCREEN_WIDTH / 4);
        let y = rng.gen_range(-100..-10);
        let side_count = rng.gen_range(6..10);
        let radius = rng.gen_range(10..20);
        let angle = rng.gen_range(10..30) as f64;
        Meteor::new(Point::new(x, y), side_count, radius, angle, true)
    }

    fn respawn_meteors(&mut self) {
        if self.meteors.is_empty() {
            for _ in 1..=MAX_METEORS_COUNT {
                let m = Self::spawn_random_meteor();
                self.meteors.push(m);
            }
        }
    }

    fn check_meteor_shuttle_collisions(&self) -> bool {
        for m in self.meteors.iter() {
            if self.shuttle.check_collision_with_meteor(m) {
                return true;
            }
        }
        false
    }

    pub fn handle_event(&mut self, event: sdl2::event::Event) {
        match self.state {
            GameState::Menu | GameState::OutOfFuel | GameState::JobsDone | GameState::MeteorHit => {
                if let sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } = event
                {
                    if let Some(command) = self.menu_commands.get(&keycode).map(|c| c.as_ref()) {
                        if let Some(new_state) = command.execute() {
                            self.state = new_state;
                        }
                    }
                }
            }
            GameState::Playing => {
                if let sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } = event
                {
                    if let Some(command) = self.play_commands.get(&keycode).map(|c| c.as_ref()) {
                        command.execute(&mut self.shuttle, self.delta_second);
                    }
                }

                if let sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } = event
                {
                    if let Some(command) = self.menu_commands.get(&keycode).map(|c| c.as_ref()) {
                        if let Some(new_state) = command.execute() {
                            self.state = new_state;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn update_game(&mut self, event_pump: &mut EventPump) -> Option<GameState> {
        for event in event_pump.poll_iter() {
            self.handle_event(event);
        }

        if self.state == GameState::Playing {
            self.calc_distances();
            self.move_meteors(self.delta_second.as_secs_f32());
            self.check_out_of_ranges();
            self.respawn_meteors();
            if self.check_meteor_shuttle_collisions() {
                self.state = GameState::MeteorHit;
                return Some(GameState::MeteorHit);
            }

            if let Some(value) = self.check_shuttle() {
                return value;
            }
        }

        None
    }

    pub fn draw_game(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for i in 0..self.mountain_points.len() - 1 {
            let start = self.mountain_points[i];
            let end = self.mountain_points[i + 1];
            canvas.set_draw_color(Color::GRAY);
            canvas.draw_line(start, end)?;
        }

        for p in &self.landing_platforms {
            p.draw(canvas)?;
        }

        for m in &self.meteors {
            m.draw(canvas)?;
        }

        self.shuttle.draw(canvas, Color::YELLOW)?;

        for s in &self.stars {
            s.draw(canvas)?;
        }

        self.hud.draw(&self.shuttle, &self.distances, canvas)?;

        canvas.present();
        Ok(())
    }
}

impl GameObject for Game {
    fn update(
        &mut self,
        event_pump: &mut EventPump,
        _randomizer: &mut ThreadRng,
        delta_time: Duration,
    ) -> Result<MainState, String> {
        self.delta_second = delta_time;

        match self.state {
            GameState::Menu => {
                if let Some(new_state) = self.update_game(event_pump) {
                    self.state = new_state;
                }
            }
            GameState::NewGame => {
                *self = Game::new();
                self.state = GameState::Playing;
            }
            GameState::Playing => {
                if let Some(game_state) = self.update_game(event_pump) {
                    self.state = game_state;
                }
            }
            GameState::OutOfFuel | GameState::JobsDone | GameState::MeteorHit => {
                self.meteors.clear();
                if let Some(new_state) = self.update_game(event_pump) {
                    self.state = new_state;
                }
            }
            GameState::ExitGame => return Ok(MainState::Exit),
        }

        Ok(MainState::Running)
    }

    fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        _texture_manager: &AssetManager,
    ) -> Result<(), String> {
        match self.state {
            GameState::Menu => {
                MainMenu::draw(canvas)?;
                canvas.present();
            }
            GameState::OutOfFuel | GameState::JobsDone | GameState::MeteorHit => {
                GameOverMenu::draw(self, canvas)?;
                canvas.present();
            }
            _ => {
                self.draw_game(canvas)?;
            }
        }
        Ok(())
    }
}
