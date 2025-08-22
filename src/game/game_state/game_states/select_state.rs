use rand::seq::IndexedRandom;
use std::{
    cell::RefCell,
    collections::HashMap,
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
    game_info::{make_map, GameInfo},
    game_object::game_objects::bud::InitialBudData,
    game_state::{game_states::GameStateEnum, GameState},
    menu::menu_state::menu_states::{
        select_bud_state::SelectBudState, MenuStateEnum, MenuStateHandler,
    },
};

pub struct SelectState<'g> {
    buttons: Vec<MenuButton<GameInfo<'g>>>,
    select_info: Rc<RefCell<SelectInfo<'g>>>,
    msh: MenuStateHandler<'g>,
}

pub struct SelectInfo<'g> {
    pub initial_buds_tuple: (Vec<InitialBudData<'g>>, Vec<InitialBudData<'g>>),
    pub current_bud: Option<usize>,
    pub trait_description: String,
    pub team: u8,
    pub done: bool,
    pub icon_textures: HashMap<String, Rc<Texture<'g>>>,
}

impl<'g> SelectInfo<'g> {
    pub fn get_current_initial_bud_data(&mut self) -> Option<&mut InitialBudData<'g>> {
        if let Some(current_bud) = self.current_bud {
            let initial_buds_tuple = if self.team == 0 {
                &mut self.initial_buds_tuple.0
            } else {
                &mut self.initial_buds_tuple.1
            };
            return Some(&mut initial_buds_tuple[current_bud]);
        } else {
            None
        }
    }
}

impl<'g> SelectState<'g> {
    pub fn new(initial_buds_tuple: (Vec<InitialBudData<'g>>, Vec<InitialBudData<'g>>)) -> Self {
        // let initial_buds_tuple = (
        //     Rc::new(RefCell::new(initial_buds_tuple.0.clone())),
        //     Rc::new(RefCell::new(initial_buds_tuple.1.clone())),
        // );
        let mut buttons = Vec::new();
        // buttons.push(MenuButton::new(
        //     Rect::new(100, 100, 100, 200),
        //     "Start",
        //     Box::new(|gi: &mut GameInfo| {
        //         // gi.game_state_handler
        //         //     .new_state(GameStateEnum::Arena(Rc::clone(&initial_buds_tuple_2)));
        //     }),
        // ));
        Self {
            buttons,
            select_info: Rc::new(RefCell::new(SelectInfo {
                team: 0,
                current_bud: None,
                trait_description: String::new(),
                initial_buds_tuple: initial_buds_tuple.clone(),
                done: false,
                icon_textures: HashMap::new(),
            })),
            msh: MenuStateHandler::new(),
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
        team: u8,
        tex: Rc<Texture<'g>>,
        name_generator: &NameGenerator,
    ) {
        while initial_buds.len() < 5 {
            initial_buds.push(InitialBudData::default(
                Rc::clone(&tex),
                team,
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

        self.select_info.borrow_mut().icon_textures = make_map(
            "assets/icons",
            &|file| Rc::new(gi.texture_creator.load_texture(&file).unwrap()),
            &["png", "jpg", "jpeg"],
        );

        Self::setup_buds(
            &mut self.select_info.borrow_mut().initial_buds_tuple.0,
            0,
            Rc::clone(&tex),
            &name_generator,
        );
        Self::setup_buds(
            &mut self.select_info.borrow_mut().initial_buds_tuple.1,
            1,
            Rc::clone(&tex),
            &name_generator,
        );
        self.msh.add_menu_states(Box::new([(
            MenuStateEnum::InitialBudDatas((Rc::clone(&self.select_info))),
            Box::new(SelectBudState::new(gi)),
        )]));
        self.msh
            .load_menu(MenuStateEnum::InitialBudDatas(Rc::clone(&self.select_info)));
    }
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        let mouse_state = gi.input.mouse_state.clone();
        // for button in self.buttons.iter_mut() {
        //     if button.press(&mouse_state, gi, None).1 {
        //         gi.game_state_handler.new_state(GameStateEnum::Arena((
        //             self.select_info.borrow().initial_buds_tuple.0.clone(),
        //             self.select_info.borrow().initial_buds_tuple.1.clone(),
        //         )));
        //     }
        //     button.draw(canvas, &gi.camera);
        // }
        if self.select_info.borrow().done {
            gi.game_state_handler.new_state(GameStateEnum::Arena((
                self.select_info.borrow().initial_buds_tuple.0.clone(),
                self.select_info.borrow().initial_buds_tuple.1.clone(),
            )));
        }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        canvas.string(0, 0, "Select", sdl2::pixels::Color::RGB(0, 255, 0));

        self.msh.handle_state(gi, delta_time, canvas);
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
