use sdl2::{render::Canvas, video::Window, EventPump};

use crate::game::{game_info::GameInfo, menu::menu_state::menu_states::MenuStateEnum};

pub mod menu_states;

pub trait MenuState<'g> {
    fn start(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {}
    fn load(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        menu_state_enum: &MenuStateEnum<'g>,
    ) {
    }
    /// True for hover over ui, false for no hover
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) -> bool {
        false
    }
    fn quit(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {}
}

// pub struct State<'g> {
//     pub scene_manager: SceneManager<'g>,
//     pub collisions: Collisions<'g>,
// }
