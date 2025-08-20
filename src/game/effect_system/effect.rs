use std::{cell::RefCell, rc::Rc};

use crate::game::game_object::game_objects::bud::{Bud, BudData};

pub struct Tile {
    effects: Vec<Rc<RefCell<dyn Effect>>>,
}
impl Tile {
    pub fn add_effect(&mut self, eff: Rc<RefCell<dyn Effect>>) {
        self.effects.push(eff);
    }
    pub fn get_effects(&mut self) -> Vec<Rc<RefCell<dyn Effect>>> {
        self.effects.retain(|eff| eff.borrow().is_active());
        return self.effects.clone();
    }
}

pub trait Effect {
    fn is_active(&self) -> bool;
    fn apply(&self, bud: &mut BudData);
}
