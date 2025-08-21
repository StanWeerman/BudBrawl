use std::{cell::RefCell, rc::Rc};

use sdl2::{
    gfx::primitives::DrawRenderer, keyboard::Keycode, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

use crate::game::{
    button::{Button, MenuButton},
    game_info::GameInfo,
    game_object::game_objects::bud::InitialBudData,
    game_state::{game_states::GameStateEnum, GameState},
};

pub struct SelectState<'g> {
    buttons: Vec<MenuButton<GameInfo<'g>>>,
    initial_buds_tuple: (Vec<InitialBudData<'g>>, Vec<InitialBudData<'g>>),
}

impl<'g> SelectState<'g> {
    pub fn new(initial_buds_tuple: (Vec<InitialBudData<'g>>, Vec<InitialBudData<'g>>)) -> Self {
        // println!("Making Home");
        // let initial_buds_tuple_1 = Rc::new(RefCell::new(initial_buds_tuple.clone()));
        // let initial_buds_tuple_2 = Rc::clone(&initial_buds_tuple);
        let mut buttons = Vec::new();
        buttons.push(MenuButton::new(
            Rect::new(100, 100, 100, 200),
            "Start",
            Box::new(|gi: &mut GameInfo| {
                // gi.game_state_handler
                //     .new_state(GameStateEnum::Arena(Rc::clone(&initial_buds_tuple_2)));
            }),
        ));
        Self {
            buttons,
            initial_buds_tuple: initial_buds_tuple.clone(),
        }
    }
    pub fn new_state(state: &GameStateEnum<'g>) -> Box<dyn GameState<'g> + 'g> {
        match state {
            GameStateEnum::Select(initial_buds_tuple) => {
                Box::new(Self::new(initial_buds_tuple.clone()))
            }
            _ => unreachable!(),
        }
    }
}

impl<'g> GameState<'g> for SelectState<'g> {
    // fn start(
    //     &mut self,
    //     gi: &mut GameInfo<'g>,
    //     delta_time: f32,
    //     canvas: &mut Canvas<Window>,
    //     event_pump: &mut EventPump,
    // ) {
    // }
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        let mouse_state = gi.input.mouse_state.clone();
        for button in self.buttons.iter_mut() {
            if button.press(&mouse_state, gi, None).1 {
                gi.game_state_handler
                    .new_state(GameStateEnum::Arena(self.initial_buds_tuple.clone()));
            }
            button.draw(canvas, &gi.camera);
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        canvas.draw_rect(Rect::new(10, 10, 100, 100));
        canvas.string(0, 0, "Select", sdl2::pixels::Color::RGB(0, 255, 0));
    }
}
