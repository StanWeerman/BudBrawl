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

    fn apply(&mut self, bud: Rc<RefCell<BudData<'g>>>) {
        todo!()
    }
}

pub struct DamageEffect {
    applications: i32,
    damage: u16,
}
impl DamageEffect {
    pub fn new(damage: u16) -> Self {
        Self {
            applications: 1,
            damage,
        }
    }
}

impl<'g> Effect<'g> for DamageEffect {
    fn is_active(&self) -> bool {
        self.applications > 0
    }

    fn apply(&mut self, bud: Rc<RefCell<BudData<'g>>>) {
        self.applications -= 1;
        bud.borrow_mut().remove_health(self.damage);
    }
}
