use rand::seq::IndexedRandom;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
};

use sdl2::{
    gfx::primitives::DrawRenderer,
    image::LoadTexture,
    keyboard::Keycode,
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
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
    fn setup_buds(
        initial_buds: &mut Vec<InitialBudData<'g>>,
        tex: Rc<Texture<'g>>,
        name_generator: &NameGenerator,
    ) {
        while initial_buds.len() < 5 {
            initial_buds.push(InitialBudData::default(
                Rc::clone(&tex),
                initial_buds.len() as u8,
                name_generator,
            ));
        }
    }
}

impl<'g> GameState<'g> for SelectState<'g> {
    fn start(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        canvas: &mut Canvas<Window>,
        event_pump: &mut EventPump,
    ) {
        let tex = Rc::new(
            gi.texture_creator
                .load_texture(&"assets/bud_2.png")
                .unwrap(),
        );
        let name_generator = NameGenerator::new("assets/names/names.txt");

        Self::setup_buds(
            &mut self.initial_buds_tuple.0,
            Rc::clone(&tex),
            &name_generator,
        );
        Self::setup_buds(
            &mut self.initial_buds_tuple.1,
            Rc::clone(&tex),
            &name_generator,
        );
        // while self.initial_buds_tuple.0.len() < 5 {
        //     self.initial_buds_tuple.0.push(InitialBudData::default(
        //         Rc::clone(&tex),
        //         self.initial_buds_tuple.0.len() as u8,
        //     ));
        // }
        // while self.initial_buds_tuple.1.len() < 5 {
        //     self.initial_buds_tuple
        //         .1
        //         .push(InitialBudData::default(Rc::clone(&tex)));
        // }
    }
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

pub struct NameGenerator {
    names: Vec<String>,
}

impl NameGenerator {
    fn new(file: impl AsRef<Path>) -> Self {
        let names = NameGenerator::lines_from_file(file);
        Self { names }
    }
    pub fn selectRandName(&self) -> String {
        match self.names.choose(&mut rand::rng()) {
            Some(i) => return i.to_string(),
            None => return "Hello".to_string(),
        }
    }

    fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()
    }
}
