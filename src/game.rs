use std::{cell::RefCell, collections::HashMap, rc::Rc};

use camera::Camera;
use input::Input;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    mixer::Music,
    render::{Canvas, TextureCreator},
    sys::{rand, SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency},
    video::{Window, WindowContext},
};

pub mod collision_system;
pub mod effect_system;
pub mod game_object;
pub mod turn_system;

pub mod camera;

pub mod input;

pub mod game_info;

pub mod game_state;

pub mod scene_manager;

pub mod menu;

pub mod button;

use game_info::GameInfo;

use crate::{
    game::game_state::{
        game_states::{home_state::HomeState, GameStateEnum},
        GameState,
    },
    vector2d::Vector2d,
};

pub struct Game<'t> {
    gi: GameInfo<'t>,
    canvas: Canvas<Window>,
    event_pump: sdl2::EventPump,
    game_state: Box<dyn GameState<'t> + 't>,
}
impl<'t> Game<'t> {
    pub fn new(
        canvas: Canvas<Window>,
        event_pump: sdl2::EventPump,
        texture_creator: &'t TextureCreator<WindowContext>,
    ) -> Self {
        let gi = GameInfo::new(&canvas, texture_creator);
        let mut game_states: HashMap<String, Box<dyn GameState<'t>>> = HashMap::new();
        Self {
            gi,
            canvas,
            event_pump,
            game_state: HomeState::new_state(&GameStateEnum::Home(sdl2::pixels::Color::RGB(
                255, 0, 0,
            ))),
        }
    }
    pub fn run(&mut self) -> Result<(), String> {
        let mut time_before = unsafe { SDL_GetPerformanceCounter() };

        while self.gi.running {
            self.gi
                .input
                .get_input(&mut self.event_pump, &mut self.gi.running);
            self.gi.camera.set_window(&mut self.canvas);
            let time_after = unsafe { SDL_GetPerformanceCounter() };
            let delta_time: f32 = (time_after - time_before) as f32 * 1000.0
                / unsafe { SDL_GetPerformanceFrequency() as f32 };
            time_before = time_after;

            if self
                .gi
                .game_state_handler
                .handle_state(&mut self.game_state)
            {
                self.game_state.start(
                    &mut self.gi,
                    delta_time,
                    &mut self.canvas,
                    &mut self.event_pump,
                );
            }

            self.game_state
                .run(&mut self.gi, delta_time, &mut self.canvas);

            self.canvas
                .set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            self.canvas.present();
            self.canvas.clear();
        }
        Ok(())
    }
}
