use std::{cell::RefCell, rc::Rc};

use crate::game::{
    collision_system::collisions::Colliding, effect_system::effects::Effect,
    game_object::game_objects::bud::bud_data::BudData,
};

#[derive(Clone)]
pub struct SelfEffect {}

impl SelfEffect {
    pub fn new() -> SelfEffect {
        SelfEffect {}
    }
}

impl<'g> Effect<'g> for SelfEffect {
    fn is_active(&self) -> bool {
        // todo!()
        true
    }

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        // todo!()
    }
}

#[derive(Clone)]
pub struct DamageEffect {
    applications: i32,
    damage: u16,
}

#[derive(Clone)]
pub struct FighterEffect {
    applications: i32,
    damage: u16,
}

impl FighterEffect {
    pub fn new() -> Self {
        Self {
            applications: 1,
            damage: 1,
        }
    }
}

impl<'g> Effect<'g> for FighterEffect {
    fn is_active(&self) -> bool {
        self.applications > 0
    }

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        self.applications -= 1;
        bud.borrow_mut().damage += self.damage;
    }
}
#[derive(Clone)]
pub struct BulwarkEffect {
    applications: i32,
    health: u16,
}

impl BulwarkEffect {
    pub fn new() -> Self {
        Self {
            applications: 1,
            health: 1,
        }
    }
}

impl<'g> Effect<'g> for BulwarkEffect {
    fn is_active(&self) -> bool {
        self.applications > 0
    }

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        self.applications -= 1;
        bud.borrow_mut().max_health += self.health;
    }
}

#[derive(Clone)]
pub struct ScoutEffect {
    applications: i32,
    speed: u16,
}

impl ScoutEffect {
    pub fn new() -> Self {
        Self {
            applications: 1,
            speed: 1,
        }
    }
}

impl<'g> Effect<'g> for ScoutEffect {
    fn is_active(&self) -> bool {
        self.applications > 0
    }

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        self.applications -= 1;
        bud.borrow_mut().speed += self.speed;
    }
}

#[derive(Clone)]
pub struct MendingEffect {
    applications: i32,
    healing: u16,
}

impl MendingEffect {
    pub fn new() -> Self {
        Self {
            applications: 1,
            healing: 1,
        }
    }
}

impl<'g> Effect<'g> for MendingEffect {
    fn is_active(&self) -> bool {
        self.applications > 0
    }

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        println!("how many apps");
        self.applications -= 1;
        bud.borrow_mut().add_health(self.healing);
    }
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

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        self.applications -= 1;
        bud.borrow_mut().remove_health(self.damage);
    }
}
