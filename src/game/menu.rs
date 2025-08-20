pub mod menu_info;

pub mod menu_state;

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    rc::Rc,
};

use sdl2::{
    gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas,
    sys::False, video::Window, EventPump,
};

use crate::game::{
    camera::Camera, game_info::GameInfo, input::Input,
    menu::menu_state::menu_states::MenuStateHandler,
};

pub struct Menu<'g> {
    // menu_info: MenuInfo,
    // menu_system: MenuSystem,
    menu_state_handler: MenuStateHandler<'g>,
}
impl<'g> Menu<'g> {
    pub fn new(gi: &mut GameInfo<'g>) -> Self {
        Self {
            menu_state_handler: MenuStateHandler::new(),
        }
    }
    pub fn update(&mut self, e: &sdl2::EventPump, gi: &mut GameInfo, input: &Input) {
        // self.menu_state_handler
        //     .handle_state(gi, delta_time, canvas, menu_state_enum);
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>, camera: &Camera) {}
}
