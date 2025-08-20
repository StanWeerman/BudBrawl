use std::{cell::RefCell, rc::Rc};

use crate::game::game_object::game_objects::bud::{Bud, BudData};

pub struct Tile<'g> {
    effects: Vec<Rc<RefCell<dyn Effect<'g> + 'g>>>,
}

pub trait Effect<'g> {
    fn is_active(&self) -> bool;
    fn apply(&self, bud: Rc<RefCell<BudData<'g>>>);
}
