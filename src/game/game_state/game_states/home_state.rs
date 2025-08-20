use sdl2::{keyboard::Keycode, rect::Rect, render::Canvas, video::Window, EventPump};

use crate::game::{
    button::{Button, MenuButton},
    game_info::GameInfo,
    game_state::{game_states::GameStateEnum, GameState},
};

pub struct HomeState<'h> {
    buttons: Vec<MenuButton<GameInfo<'h>>>,
    color: sdl2::pixels::Color,
}

impl<'g> HomeState<'g> {
    pub fn new(color: &sdl2::pixels::Color) -> Self {
        // println!("Making Home");
        let mut buttons = Vec::new();
        buttons.push(MenuButton::new(
            Rect::new(100, 100, 100, 200),
            "Start",
            Box::new(|gi: &mut GameInfo| {
                gi.game_state_handler.new_state(GameStateEnum::Zoom);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(200, 100, 100, 200),
            "SchipTest",
            Box::new(|gi: &mut GameInfo| {
                gi.game_state_handler.new_state(GameStateEnum::Testing);
            }),
        ));
        Self {
            buttons,
            color: color.clone(),
        }
    }
    pub fn new_state(state: &GameStateEnum) -> Box<dyn GameState<'g> + 'g> {
        match state {
            GameStateEnum::Home(color) => Box::new(Self::new(color)),
            _ => unreachable!(),
        }
    }
}

impl<'g> GameState<'g> for HomeState<'g> {
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
        canvas.set_draw_color(self.color);
        canvas.draw_rect(Rect::new(10, 10, 100, 100));
    }
}
