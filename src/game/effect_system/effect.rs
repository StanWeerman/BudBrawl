use std::{cell::RefCell, rc::Rc};

use crate::game::game_object::game_objects::bud::{Bud, BudData};

pub struct Tile<'g> {
    effects: Vec<Rc<RefCell<dyn Effect<'g> + 'g>>>,
}

pub trait Effect<'g> {
    fn is_active(&self) -> bool;
    fn apply(&self, bud: Rc<RefCell<BudData<'g>>>);
}

pub struct SelfEffect {}

impl SelfEffect {
    pub fn new() -> SelfEffect {
        SelfEffect {}
    }
}

impl<'g> Effect<'g> for SelfEffect {
    fn is_active(&self) -> bool {
        todo!()
    }

    fn apply(&self, bud: Rc<RefCell<BudData<'g>>>) {
        todo!()
    }
}
