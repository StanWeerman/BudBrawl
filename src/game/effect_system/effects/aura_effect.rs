use std::{cell::RefCell, rc::Rc};

use dyn_clone::DynClone;

use crate::game::{
    collision_system::collisions::Colliding,
    effect_system::effects::{self_effect::DamageEffect, Effect},
    game_object::game_objects::bud::bud_data::BudData,
};
#[derive(Clone)]
pub struct AuraEffect<'r> {
    applications: i32,
    effect: Box<dyn Effect<'r>>,
}
impl<'g> AuraEffect<'g> {
    pub fn new(effect: Box<dyn Effect<'g>>) -> Self {
        Self {
            applications: 2,
            effect,
        }
    }
}
// impl<'g> DynClone for AuraEffect<'g> {
//     fn __clone_box(&self, _: Private) -> *mut () {
//         todo!()
//     }
// }

impl<'g> Effect<'g> for AuraEffect<'g> {
    fn is_active(&self) -> bool {
        self.applications > 0
    }

    fn apply(
        &mut self,
        bud: Rc<RefCell<BudData<'g>>>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        self.applications -= 1;
        for other_bud in others.clone() {
            println!("????");
            if let Ok(mut temp) = other_bud.try_borrow_mut() {
                println!("This should deal damage?");
                temp.on_effected(dyn_clone::clone_box(&*self.effect), others.clone());
            }
        }
    }
}
