use sdl2::{
    gfx::primitives::DrawRenderer, keyboard::Keycode, rect::Rect, render::Canvas, video::Window,
    EventPump,
};

use crate::game::{
    button::{Button, MenuButton},
    game_info::GameInfo,
    game_state::{game_states::GameStateEnum, GameState},
};

pub struct SelectState<'h> {
    buttons: Vec<MenuButton<GameInfo<'h>>>,
}

impl<'g> SelectState<'g> {
    pub fn new() -> Self {
        // println!("Making Home");
        let mut buttons = Vec::new();
        buttons.push(MenuButton::new(
            Rect::new(100, 100, 100, 200),
            "Start",
            Box::new(|gi: &mut GameInfo| {
                gi.game_state_handler.new_state(GameStateEnum::Arena);
            }),
        ));
        // buttons.push(MenuButton::new(
        //     Rect::new(200, 100, 100, 200),
        //     "SchipTest",
        //     Box::new(|gi: &mut GameInfo| {
        //         gi.game_state_handler.new_state(GameStateEnum::Testing);
        //     }),
        // ));
        Self { buttons }
    }
    pub fn new_state(state: &GameStateEnum) -> Box<dyn GameState<'g> + 'g> {
        match state {
            GameStateEnum::Select => Box::new(Self::new()),
            _ => unreachable!(),
        }
    }
}

impl<'g> GameState<'g> for SelectState<'g> {
    // fn start(
    //     &mut self,
    //     gi: &mut &GameInfo<'g>,
    //     delta_time: f32,
    //     canvas: &mut Canvas<Window>,
    //     event_pump: &mut EventPump,
    // ) {
    // }
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        let mouse_state = gi.input.mouse_state.clone();
        for button in self.buttons.iter_mut() {
            button.press(&mouse_state, gi, None);
            button.draw(canvas, &gi.camera);
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        canvas.draw_rect(Rect::new(10, 10, 100, 100));
        canvas.string(0, 0, "Select", sdl2::pixels::Color::RGB(0, 255, 0));
    }
}
