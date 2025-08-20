use std::{cell::RefCell, rc::Rc};

use sdl2::{render::Canvas, video::Window, EventPump};

use crate::game::{
    game_info::GameInfo,
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
}

impl<'g> StateInfo<'g> {
    pub fn new() -> Self {
        Self {
            add_list: Vec::new(),
        }
    }
    pub fn add_object(&mut self, object: Rc<RefCell<Object<'g>>>) {
        self.add_list.push(object);
    }
    pub fn add_objects(&mut self, scene_manager: &mut SceneManager<'g>) {
        scene_manager.object_list.extend(self.add_list.drain(0..));
    }
}

// pub struct State<'g> {
//     pub scene_manager: SceneManager<'g>,
//     pub collisions: Collisions<'g>,
// }
