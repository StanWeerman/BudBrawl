use crate::game::{
    collision_system::collisions::Collisions, game_info::GameInfo, game_state::StateInfo,
};

pub mod death_match;

pub trait GameMode<'g> {
    fn setup(
        &mut self,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) {
    }

    //
    fn check_done(
        &mut self,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    );
    fn draw(&mut self, collisions: &mut Collisions, gi: &mut GameInfo<'g>, si: &mut StateInfo<'g>) {
    }
}

// pub enum RoundState {
//     Win(Team),
//     Continue,
// }

// pub enum Team {
//     One,
//     Two,
// }

// impl Team {
//     pub fn from_index(index: u8) -> Team {
//         match index {
//             0 => Team::One,
//             _ => Team::Two,
//         }
//     }
//     pub fn to_index(&self) -> u8 {
//         match self {
//             Team::One => 0,
//             Team::Two => 1,
//         }
//     }
// }
