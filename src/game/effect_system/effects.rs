use std::{cell::RefCell, rc::Rc};

use dyn_clone::DynClone;

use crate::game::{
    collision_system::collisions::Colliding,
    game_object::game_objects::bud::{bud_data::BudData, Bud},
};

pub mod aura_effect;
pub mod self_effect;

pub struct Tile<'g> {
    effects: Vec<Rc<RefCell<dyn Effect<'g> + 'g>>>,
}

pub trait Effect<'g>: DynClone {
    fn is_active(&self) -> bool;
    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    );
}
impl<'g> Clone for Box<dyn Effect<'g>> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}
