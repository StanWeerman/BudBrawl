use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use sdl2::{
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
    EventPump,
};

use crate::{
    game::{
        button::Button,
        camera::Camera,
        collision_system::collisions::{Colliding, Collisions, Side},
        effect_system::effect::Effect,
        game_info::GameInfo,
        game_object::{game_objects::GameObjectEnum, GameObject, SuperGameObject},
        game_state::StateInfo,
        input::Input,
        menu::menu_state::menu_states::{MenuStateEnum, MenuStateHandler},
    },
    vector2d::Vector2d,
};

pub struct Bud<'g> {
    position: Point,
    bud_data: Rc<RefCell<BudData<'g>>>,
    hovered: bool,
    pressed: bool,
    effects: Vec<Rc<RefCell<dyn Effect<'g> + 'g>>>,
    moved: [bool; 4],
}
impl<'g> Bud<'g> {
    pub fn new(position: Point, initial_blud_data: Rc<InitialBudData<'g>>) -> Self {
        Self {
            position,
            bud_data: Rc::new(RefCell::new(BudData::default(initial_blud_data))),
            hovered: false,
            pressed: false,
            effects: vec![],
            moved: [false, false, false, false],
        }
    }
    pub fn move_bud(&mut self, gi: &mut GameInfo<'g>, delta_time: f32) {
        if gi.input.is_pressed(Keycode::Up) && !self.moved[0] {
            self.moved[0] = true;
            self.position.y -= 1;
        } else if gi.input.is_released(Keycode::Up) {
            self.moved[0] = false;
        }
        if gi.input.is_pressed(Keycode::Down) && !self.moved[1] {
            self.moved[1] = true;
            self.position.y += 1
        } else if gi.input.is_released(Keycode::Down) {
            self.moved[1] = false;
        }
        if gi.input.is_pressed(Keycode::Left) && !self.moved[2] {
            self.moved[2] = true;
            self.position.x -= 1;
        } else if gi.input.is_released(Keycode::Left) {
            self.moved[2] = false;
        }
        if gi.input.is_pressed(Keycode::Right) && !self.moved[3] {
            self.moved[3] = true;
            self.position.x += 1;
        } else if gi.input.is_released(Keycode::Right) {
            self.moved[3] = false;
        }
    }
    pub fn add_effect(&mut self, eff: Rc<RefCell<dyn Effect<'g> + 'g>>) {
        self.effects.push(eff);
    }
    pub fn apply_effects(&mut self) {
        self.effects
            .iter()
            .for_each(|eff| eff.borrow().apply(Rc::clone(&self.bud_data)));
    }
}

impl<'g> GameObject<'g> for Bud<'g> {
    fn get_position(&self) -> Vector2d {
        Vector2d::new(self.position.x as f32, self.position.y as f32)
    }

    fn get_draw_values(&self) -> (Vector2d, Vector2d) {
        (
            Vector2d::new(self.position.x as f32, self.position.y as f32),
            Vector2d::new(10.0, 10.0),
        )
    }

    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        // let (position, size) = self.get_draw_values();
        canvas.set_draw_color(Color::RGBA(139, 210, 241, 255));
        let mut some_rect = Rect::from_center(self.position, 10, 10);
        camera.rect_to_camera(&mut some_rect);

        // canvas.copy_ex(
        //     &self.ship_data.borrow().initial.texture,
        //     None,
        //     some_rect,
        //     self.ship_data.borrow().angle as f64,
        //     None,
        //     false,
        //     false,
        // );
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        canvas.fill_rect(some_rect);

        // canvas.draw_point(some_rect.top_left() + Point::new(10, 0));
    }
    fn update(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
        // level_info: &mut LevelInfo<'g>,
    ) -> bool {
        self.move_bud(gi, _delta_time);

        true
    }
}

impl<'g> Button<'g> for Bud<'g> {
    fn get_pressed(&self) -> (bool, bool) {
        (self.hovered, self.pressed)
    }

    fn set_hovered(&mut self, hovered: bool) {
        self.hovered = hovered;
    }

    fn set_pressed(&mut self, pressed: bool) {
        self.pressed = pressed;
    }

    fn get_draw_values(&self) -> (Rect, &str) {
        todo!()
    }

    fn hover_action(&mut self, input: &mut Self::Input) {
        println!("Hovered");
    }

    fn action(&mut self, input: &mut Self::Input) {
        // input.load_menu(MenuStateEnum::Ship(Some(Rc::clone(&self.ship_data))));
        println!("Pressed");
    }

    fn in_bounds(&self, mouse_x: i32, mouse_y: i32, camera: Option<&Camera>) -> bool {
        let mut ret = false;
        let mut center = self.position.clone();
        if let Some(camera) = camera {
            camera.point_to_camera(&mut center);
        }
        let distance_squared =
            (center.x as i32 - mouse_x).pow(2) + (center.y as i32 - mouse_y).pow(2);
        if distance_squared < 100 {
            return true;
        }
        ret
    }

    type Input = MenuStateHandler<'g>;
}

pub struct BudData<'g> {
    initial: Rc<InitialBudData<'g>>,
    selected: bool,
    health: u16,
    speed: u16,
}

impl<'g> BudData<'g> {
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn unselect(&mut self) {
        self.selected = false;
    }

    fn default(initial: Rc<InitialBudData<'g>>) -> BudData<'g> {
        BudData {
            initial,
            selected: false,
            health: 10,
            speed: 1,
        }
    }
}

pub struct InitialBudData<'g> {
    texture: Rc<Texture<'g>>,
    // traits:
}

impl<'g> InitialBudData<'g> {
    pub fn default(texture: Rc<Texture<'g>>) -> InitialBudData {
        InitialBudData { texture }
    }
}
