use std::{cell::RefCell, rc::Rc};

use crate::game::effect_system::effect::Effect;

pub struct Bud {
    x: usize,
    y: usize,
    effects: Vec<Rc<RefCell<dyn Effect>>>,
}

impl Bud {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            effects: vec![],
        }
    }
    pub fn add_effect(&mut self, eff: Rc<RefCell<dyn Effect>>) {
        self.effects.push(eff);
    }
    pub fn apply_effects(&mut self) {
        self.effects.apply(self);
    }
}
