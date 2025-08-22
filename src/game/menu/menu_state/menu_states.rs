use std::{cell::RefCell, cmp, collections::HashMap, hash::Hash, rc::Rc};

use crate::game::{
    game_info::GameInfo,
    game_object::game_objects::bud::{BudData, InitialBudData},
    game_state::{game_states::select_state::SelectInfo, GameState},
    menu::menu_state::MenuState,
};

pub mod bud_state;
pub mod select_bud_state;

use bud_state::BudState;
use sdl2::{render::Canvas, video::Window, EventPump};

pub struct MenuStateHandler<'g> {
    pub menu_states: HashMap<MenuStateEnum<'g>, Box<dyn MenuState<'g> + 'g>>,
    pub state: Option<MenuStateEnum<'g>>,
    pub new_state: bool,
    pub new_state_wait: bool,
    pub press: bool,
    pub not_ready: bool,
}

impl<'g> MenuStateHandler<'g> {
    pub fn new() -> MenuStateHandler<'g> {
        MenuStateHandler {
            new_state: false,
            new_state_wait: false,
            press: false,
            state: None,
            menu_states: HashMap::new(),
            not_ready: false,
        }
    }

    pub fn add_menu_states(
        &mut self,
        menu_tuples: Box<[(MenuStateEnum<'g>, Box<dyn MenuState<'g> + 'g>)]>,
    ) {
        for menu_tuple in menu_tuples {
            self.menu_states.insert(menu_tuple.0, menu_tuple.1);
        }
    }

    pub fn load_menu(&mut self, new_state: MenuStateEnum<'g>) {
        self.new_state_wait = true;
        if !self.not_ready {
            self.state = Some(new_state);
            self.new_state = true;
        }
    }

    pub fn handle_state(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        canvas: &mut Canvas<Window>,
    ) {
        // self.not_ready = false;
        if let Some(menu_state_enum) = &self.state {
            if let Some(menu_state) = self.menu_states.get_mut(menu_state_enum) {
                if self.new_state {
                    menu_state.load(gi, delta_time, menu_state_enum);
                    self.new_state = false;
                }
                self.not_ready = menu_state.run(gi, delta_time, canvas);
            }
        }
        // if !self.not_ready && self.press && !gi.input.mouse_state.left() && !self.new_state_wait {
        //     // self.new_state = true;
        //     if let Some(menu_state_enum) = &self.state {
        //         if let Some(menu_state) = self.menu_states.get_mut(menu_state_enum) {
        //             menu_state.quit(gi, delta_time, canvas);
        //         }
        //     }
        //     self.state = None;
        // }
        // self.press = gi.input.mouse_state.left();
        // self.new_state_wait = false;
    }
}

pub enum MenuStateEnum<'g> {
    Bud(BudEnum<'g>),
    InitialBudDatas((Rc<RefCell<SelectInfo<'g>>>)),
}
pub enum BudEnum<'g> {
    LeftBud(Option<Rc<RefCell<BudData<'g>>>>),
    RightBud(Option<Rc<RefCell<BudData<'g>>>>),
}

impl<'g> Hash for MenuStateEnum<'g> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl<'g> PartialEq for MenuStateEnum<'g> {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl<'g> Eq for MenuStateEnum<'g> {}
