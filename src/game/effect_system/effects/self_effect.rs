use std::{cell::RefCell, rc::Rc};

use crate::game::{effect_system::effects::Effect, game_object::game_objects::bud::BudData};

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
