use rand::Rng;

use crate::game::{
    collision_system::collisions::Collisions,
    game_info::GameInfo,
    game_state::{game_states::arena_state::game_modes::GameMode, StateInfo},
};

#[derive(Debug)]
pub struct DeathMatch {
    required_kills: u8,
    kings: Vec<u8>,
}

impl DeathMatch {
    pub fn new(required_kills: u8, kings: Vec<u8>) -> DeathMatch {
        DeathMatch {
            required_kills,
            kings,
        }
    }
    pub fn random() -> DeathMatch {
        let mut rng = rand::thread_rng(); // Get a thread-local random number generator
        let required_kills = rng.random_range(1..=5);
        let kings_count = rng.random_range(0..=required_kills);
        let mut kings = Vec::new();
        for _ in 0..kings_count {
            let king = rng.random_range(0..=4);
            if !kings.contains(&king) {
                kings.push(king);
            }
        }
        DeathMatch {
            required_kills,
            kings,
        }
    }
}

impl<'g> GameMode<'g> for DeathMatch {
    fn setup(
        &mut self,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) {
        if self.kings.is_empty() {
            // DeathMatch
        } else {
            // Chess
        }
    }
    fn check_done(
        &mut self,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) {
        if self.kings.is_empty() {
            // DeathMatch
            let mut death_count = 0;
            for bud_data in si.bud_data_tuple.0.iter() {
                if !bud_data.borrow().alive() {
                    death_count += 1;
                }
            }
            if death_count >= self.required_kills {
                // Team 2 Wins
                si.end_round(gi);
            }

            let mut death_count = 0;
            for bud_data in si.bud_data_tuple.0.iter() {
                if !bud_data.borrow().alive() {
                    death_count += 1;
                }
            }
            if death_count >= self.required_kills {
                // Team 1 Wins
                si.end_round(gi);
            }
        } else {
            // Chess
            let mut death_count = 0;
            for (index, bud_data) in si.bud_data_tuple.0.iter().enumerate() {
                if !bud_data.borrow().alive() && self.kings.contains(&(index as u8)) {
                    death_count += 1;
                }
            }
            if death_count >= self.kings.len() {
                // Team 2 Wins
                si.end_round(gi);
            }

            let mut death_count = 0;
            for (index, bud_data) in si.bud_data_tuple.0.iter().enumerate() {
                if !bud_data.borrow().alive() && self.kings.contains(&(index as u8)) {
                    death_count += 1;
                }
            }
            if death_count >= self.kings.len() {
                // Team 2 Wins
                si.end_round(gi);
            }
        }
    }
    fn draw(&mut self, collisions: &mut Collisions, gi: &mut GameInfo<'g>, si: &mut StateInfo<'g>) {
        if self.kings.is_empty() {
            // DeathMatch
        } else {
            // Chess
        }
    }
}
