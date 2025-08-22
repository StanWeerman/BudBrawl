use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use crate::game::collision_system::collisions::Collisions;
use crate::game::game_info::GameInfo;
use crate::game::game_state::StateInfo;
use crate::game::menu::menu_state::menu_states::MenuStateHandler;
use crate::game::scene_manager::Object;

pub struct TurnHandler<'g> {
    object_list: VecDeque<Rc<RefCell<Object<'g>>>>,
    current: Option<Rc<RefCell<Object<'g>>>>,
}

impl<'g> TurnHandler<'g> {
    pub fn new() -> Self {
        Self {
            object_list: VecDeque::new(),
            current: None,
        }
    }
    pub fn next_turn(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
        msh: &mut MenuStateHandler<'g>,
    ) {
        let ending = self.current.clone();
        match ending {
            Some(val) => {
                val.borrow_mut().end(_delta_time, collisions, gi, si, msh);
                self.add(val);
            }
            None => {}
        }
        let starting = self.object_list.pop_front().unwrap();
        starting
            .borrow_mut()
            .start(_delta_time, collisions, gi, si, msh);
        self.current = Option::Some(starting);
    }
    pub fn add(&mut self, obj: Rc<RefCell<Object<'g>>>) {
        self.object_list.push_back(obj);
    }
    pub fn remove(&mut self, index: usize) {
        self.object_list.remove(index);
    }
    pub fn remove_all(&mut self, indexes: &[usize], collisions: &mut Collisions) {
        for (i, index) in indexes.iter().enumerate() {
            // self.object_list
            //     .get(*index - i)
            //     .unwrap()
            //     .borrow_mut()
            //     .remove_collider(collisions);
            self.object_list.remove(*index - i);
        }
    }
}
