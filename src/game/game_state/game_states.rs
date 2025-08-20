use std::{cmp, collections::HashMap, hash::Hash};

use crate::game::game_state::GameState;

pub mod arena_state;
pub mod home_state;
pub mod select_state;

use arena_state::ArenaState;
use home_state::HomeState;
use select_state::SelectState;

pub struct GameStateHandler<'g> {
    pub game_state_fns:
        HashMap<GameStateEnum, Box<dyn Fn(&GameStateEnum) -> Box<dyn GameState<'g> + 'g>>>,
    pub new_state: Option<GameStateEnum>,
    pub game_state_string: String,
}

impl<'g> GameStateHandler<'g> {
    pub fn new() -> GameStateHandler<'g> {
        let mut game_state_fns: HashMap<
            GameStateEnum,
            Box<dyn Fn(&GameStateEnum) -> Box<dyn GameState<'g>>>,
        > = HashMap::new();
        game_state_fns.insert(
            GameStateEnum::Home(sdl2::pixels::Color::RGB(0, 0, 255)),
            Box::new(HomeState::new_state),
        );
        game_state_fns.insert(GameStateEnum::Arena, Box::new(ArenaState::new_state));
        game_state_fns.insert(GameStateEnum::Select, Box::new(SelectState::new_state));

        GameStateHandler {
            new_state: None,
            game_state_fns,
            game_state_string: "HOME".to_string(),
        }
    }

    pub fn new_state(&mut self, new_state: GameStateEnum) {
        self.new_state = Some(new_state);
        // self.game_state_fn_string = Some(new_state.to_string());
    }

    pub fn handle_state(&mut self, game_state: &mut Box<dyn GameState<'g> + 'g>) -> bool {
        if let Some(new_state) = &self.new_state {
            if let Some(game_state_fn) = self.game_state_fns.get(new_state) {
                *game_state = game_state_fn(new_state);
                self.new_state = None;
                return true;
            }
        }
        false
    }
}

#[derive(Eq)]
pub enum GameStateEnum {
    Home(sdl2::pixels::Color),
    Select,
    Arena,
}

// impl Eq for GameStateEnum {}

impl Hash for GameStateEnum {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl PartialEq for GameStateEnum {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
