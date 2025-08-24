use std::{cell::RefCell, rc::Rc};

use sdl2::{render::Canvas, video::Window, EventPump};

use crate::game::{
    game_info::GameInfo,
    game_object::game_objects::bud::bud_data::BudData,
    game_state::game_states::GameStateEnum,
    scene_manager::{Object, SceneManager},
};

pub mod game_states;

pub trait GameState<'g> {
    fn start(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        canvas: &mut Canvas<Window>,
        event_pump: &mut EventPump,
    ) {
    }
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {}
}

pub struct StateInfo<'g> {
    add_list: Vec<Rc<RefCell<Object<'g>>>>,
    pub bud_data_tuple: (Vec<Rc<RefCell<BudData<'g>>>>, Vec<Rc<RefCell<BudData<'g>>>>),
}

impl<'g> StateInfo<'g> {
    pub fn new() -> Self {
        Self {
            add_list: Vec::new(),
            bud_data_tuple: (Vec::new(), Vec::new()),
        }
    }
    pub fn add_object(&mut self, object: Rc<RefCell<Object<'g>>>) {
        self.add_list.push(object);
    }
    pub fn add_objects(&mut self, scene_manager: &mut SceneManager<'g>) {
        scene_manager.object_list.extend(self.add_list.drain(0..));
    }
    pub fn end_round(&self, gi: &mut GameInfo<'g>) {
        let mut initial_buds_tuple = (Vec::new(), Vec::new());
        let mut index = 0;
        for bud_data in self.bud_data_tuple.0.iter() {
            let mut initial_bud_data = bud_data.borrow().initial.clone();
            if bud_data.borrow().alive() {
                initial_bud_data.new_round(index);
                initial_buds_tuple.0.push(initial_bud_data);
                index += 1;
            }
        }
        let mut index = 0;
        for bud_data in self.bud_data_tuple.1.iter() {
            let mut initial_bud_data = bud_data.borrow().initial.clone();
            if bud_data.borrow().alive() {
                initial_bud_data.new_round(index);
                initial_buds_tuple.1.push(initial_bud_data);
                index += 1;
            }
        }
        gi.game_state_handler
            .new_state(GameStateEnum::Select(initial_buds_tuple));
    }
}

// pub struct State<'g> {
//     pub scene_manager: SceneManager<'g>,
//     pub collisions: Collisions<'g>,
// }
