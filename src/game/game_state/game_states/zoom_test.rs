use std::{cell::RefCell, rc::Rc};

use sdl2::{
    image::LoadTexture, keyboard::Keycode, pixels::PixelFormatEnum, rect::Rect, render::Canvas,
    surface::Surface, video::Window, EventPump,
};

use crate::{
    game::{
        button::{Button, MenuButton},
        collision_system::collisions::Collisions,
        game_info::GameInfo,
        game_object::{
            game_objects::{
                ground::Ground,
                ship::{Ship, ShipData},
            },
            GameObject,
        },
        game_state::{game_states::GameStateEnum, GameState, StateInfo},
        scene_manager::SceneManager,
    },
    vector2d::Vector2d,
};

pub struct ZoomState<'g> {
    button: MenuButton<GameInfo<'g>>,
    scene_manager: SceneManager<'g>,
    si: StateInfo<'g>,
    collisions: Collisions<'g>,
    buttons: Vec<MenuButton<Ship<'g>>>,
}

impl<'g> ZoomState<'g> {
    pub fn new() -> Self {
        Self {
            scene_manager: SceneManager::new(),
            si: StateInfo::new(),
            collisions: Collisions::new(Vec::new()),
            button: MenuButton::new(
                Rect::new(100, 100, 100, 200),
                "Back",
                Box::new(|gi| {
                    gi.game_state_handler
                        .new_state(GameStateEnum::Home(sdl2::pixels::Color::RGB(0, 255, 0)));
                }),
            ),
            buttons: Vec::new(),
        }
    }
    pub fn new_state(state: &GameStateEnum) -> Box<dyn GameState<'g> + 'g> {
        let ret: Box<dyn GameState<'g>> = Box::new(Self::new());
        ret
        // match state {
        //     GameStateEnum::Testing => Box::new(Self::new()),
        //     _ => unreachable!(),
        // }
    }
}

impl<'g> GameState<'g> for ZoomState<'g> {
    fn start(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        canvas: &mut Canvas<Window>,
        event_pump: &mut EventPump,
    ) {
        let _tex = gi
            .texture_creator
            .load_texture(&"src/transport_ship_1.png")
            .unwrap();

        let tex = Rc::new(_tex);

        const X_SIZE: u32 = 100;
        const Y_SIZE: u32 = 100;

        let mut texture = gi
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, X_SIZE, Y_SIZE)
            .map_err(|e| e.to_string())
            .unwrap();
        self.scene_manager.add(Rc::new(RefCell::new(Ground::new(
            Vector2d::new(9962540 as f32, 10036960 as f32),
            20,
            texture,
        ))));
        // self.scene_manager.add(Rc::new(RefCell::new(Ship::new(
        //     Vector2d::new(100.0, 100.0),
        //     Rc::clone(&ship_data),
        // ))));
        // self.scene_manager.add(Rc::new(RefCell::new(Ship::new(
        //     Vector2d::new(200.0, 200.0),
        //     Rc::clone(&ship_data),
        // ))));
    }
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        let mouse_state = gi.input.mouse_state;
        self.button.press(&mouse_state, gi, None);

        self.scene_manager
            .update(delta_time, &mut self.collisions, gi, &mut self.si);
        self.scene_manager.draw(canvas, &mut gi.camera);
        self.button.draw(canvas, &gi.camera);
        for button in self.buttons.iter_mut() {
            // button.press(event_pump, &mut self.ships[0].borrow_mut());
            button.draw(canvas, &gi.camera);
        }
    }
}
