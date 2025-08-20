use std::{cell::RefCell, rc::Rc};

use sdl2::{
    gfx::primitives::DrawRenderer,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
    EventPump,
};

use crate::{
    game::{
        button::{Button, MenuButton},
        collision_system::collisions::Collisions,
        game_info::GameInfo,
        game_object::{
            game_objects::{bud::Bud, ground::Ground},
            GameObject,
        },
        game_state::{game_states::GameStateEnum, GameState, StateInfo},
        menu::menu_state::menu_states::MenuStateHandler,
        scene_manager::SceneManager,
    },
    vector2d::Vector2d,
};

pub struct ArenaState<'g> {
    button: MenuButton<GameInfo<'g>>,
    scene_manager: SceneManager<'g>,
    si: StateInfo<'g>,
    collisions: Collisions<'g>,
    msh: MenuStateHandler<'g>,
    view: View,
}

impl<'g> ArenaState<'g> {
    pub fn new() -> Self {
        Self {
            scene_manager: SceneManager::new(),
            si: StateInfo::new(),
            collisions: Collisions::new(Vec::new()),
            button: MenuButton::new(
                Rect::new(0, 0, 40, 20),
                "Back",
                Box::new(|gi| {
                    gi.game_state_handler
                        .new_state(GameStateEnum::Home(sdl2::pixels::Color::RGB(0, 255, 0)));
                }),
            ),
            msh: MenuStateHandler::new(),
            view: View::new(),
        }
    }
    pub fn new_state(state: &GameStateEnum) -> Box<dyn GameState<'g> + 'g> {
        match state {
            GameStateEnum::Arena => Box::new(Self::new()),
            _ => unreachable!(),
        }
    }
}

impl<'g> GameState<'g> for ArenaState<'g> {
    fn start(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        canvas: &mut Canvas<Window>,
        event_pump: &mut EventPump,
    ) {
        const X_SIZE: u32 = 100;
        const Y_SIZE: u32 = 100;
        let mut texture = gi
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, X_SIZE, Y_SIZE)
            .map_err(|e| e.to_string())
            .unwrap();
        self.scene_manager.add(Rc::new(RefCell::new(Ground::new(
            Vector2d::new(0 as f32, 0 as f32),
            2,
            texture,
        ))));

        let tex = Rc::new(
            gi.texture_creator
                .load_texture(&"../../../assets/fletcher.png")
                .unwrap(),
        );

        let mut bud = Bud::new(Point::new(0, 0));
        let _bud = Rc::new(RefCell::new(bud));
        let __bud = Rc::clone(&_bud);
        // self.collisions.add(__bud);
        self.scene_manager.add(_bud);
        // let mut ship = Ship::new(Vector2d::new(100.0, 200.0), Rc::clone(&ship_data));
        // ship.rotate_points();
        // let _ship = Rc::new(RefCell::new(ship));
        // let __ship = Rc::clone(&_ship);
        // self.collisions.add(__ship);
        // self.scene_manager.add(_ship);

        self.msh.add_menu_states(gi);
    }
    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        let mouse_state = gi.input.mouse_state.clone();
        self.view.move_view(delta_time, gi);
        gi.camera.set_window(canvas);
        self.button.press(&mouse_state, gi, None);
        self.button.draw(canvas, &gi.camera);

        self.scene_manager
            .update(delta_time, &mut self.collisions, gi, &mut self.si);
        self.scene_manager.draw(canvas, &mut gi.camera);
        self.scene_manager.press(gi, &mut self.msh);
        self.msh.handle_state(gi, delta_time, canvas);

        let mut pos = gi.camera.position.clone();
        let blocks = gi.camera.blocks();
        pos.x += blocks.0 as f32 / 2.0 * gi.camera.scale as f32;
        pos.y += blocks.1 as f32 / 2.0 * gi.camera.scale as f32;
        canvas.string(
            10,
            10,
            &format!("X: {},Y:{}", pos.x as i16, pos.y as i16,),
            Color::RGB(255, 255, 255),
        );
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        canvas.fill_rect(Rect::from_center(
            Point::new(
                (gi.camera.window_size.0 as f32 / 2.0) as i32,
                (gi.camera.window_size.1 as f32 / 2.0) as i32,
            ),
            gi.camera.window_scale() as u32,
            gi.camera.window_scale() as u32,
        ));
        self.si.add_objects(&mut self.scene_manager);

        canvas.string(0, 0, "Home", sdl2::pixels::Color::RGB(0, 255, 0));
    }
}

pub struct View {
    do_zoom_in: bool,
    do_zoom_out: bool,
}
impl View {
    pub fn new() -> Self {
        Self {
            do_zoom_in: false,
            do_zoom_out: false,
        }
    }
    pub fn move_view(&mut self, delta_time: f32, gi: &mut GameInfo) {
        // gi.camera.set_camera_mouse(
        //     delta_time,
        //     &Vector2d::new(
        //         gi.input.mouse_state.x() as f32,
        //         gi.input.mouse_state.y() as f32,
        //     ),
        // );

        let mut position = gi.camera.position.clone();
        let mut scale = gi.camera.scale;
        let blocks = gi.camera.blocks();
        let mut position = Vector2d::new(
            position.x + blocks.0 as f32 / 2.0 * scale as f32,
            position.y + blocks.1 as f32 / 2.0 * scale as f32,
        );

        if gi.input.is_pressed(Keycode::W) {
            position.y -= 0.06 * delta_time * scale as f32;
        }
        if gi.input.is_pressed(Keycode::S) {
            position.y += 0.06 * delta_time * scale as f32;
        }
        if gi.input.is_pressed(Keycode::A) {
            position.x -= 0.06 * delta_time * scale as f32;
        }
        if gi.input.is_pressed(Keycode::D) {
            position.x += 0.06 * delta_time * scale as f32;
        }
        if gi.input.is_pressed(Keycode::Up) && self.do_zoom_out {
            scale *= 2;
            self.do_zoom_out = false;
        } else if gi.input.is_released(Keycode::Up) {
            self.do_zoom_out = true;
        }
        if gi.input.is_pressed(Keycode::Down) && scale > 1 && self.do_zoom_in {
            scale /= 2;
            self.do_zoom_in = false
        } else if gi.input.is_released(Keycode::Down) {
            self.do_zoom_in = true;
        }
        // gi.camera.position = position;
        // gi.camera.scale = scale;
        let blocks = gi.camera.blocks();
        let position = Vector2d::new(
            position.x - blocks.0 as f32 / 2.0 * scale as f32,
            position.y - blocks.1 as f32 / 2.0 * scale as f32,
        );
        gi.camera.set_camera(1.0, &position, scale);
    }
}
