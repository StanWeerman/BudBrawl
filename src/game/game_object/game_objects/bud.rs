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
    active: bool,
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
            active: false,
        }
    }
    pub fn decide_move(&mut self, gi: &mut GameInfo<'g>, delta_time: f32) {
        if self.bud_data.borrow().speed > 0 {
            let mut moving = Point::new(0, 0);
            if gi.input.is_pressed(Keycode::W) && !self.moved[0] {
                self.moved[0] = true;
                moving.x = 0;
                moving.y = -1;
            } else if gi.input.is_released(Keycode::W) {
                self.moved[0] = false;
            }
            if gi.input.is_pressed(Keycode::S) && !self.moved[1] {
                self.moved[1] = true;
                moving.x = 0;
                moving.y = 1;
            } else if gi.input.is_released(Keycode::S) {
                self.moved[1] = false;
            }
            if gi.input.is_pressed(Keycode::A) && !self.moved[2] {
                self.moved[2] = true;
                moving.x = -1;
                moving.y = 0;
            } else if gi.input.is_released(Keycode::A) {
                self.moved[2] = false;
            }
            if gi.input.is_pressed(Keycode::D) && !self.moved[3] {
                self.moved[3] = true;
                moving.x = 1;
                moving.y = 0;
            } else if gi.input.is_released(Keycode::D) {
                self.moved[3] = false;
            }
            println!("{}, {}", moving.x, moving.y);
            self.move_bud(moving, delta_time);
        }
    }
    pub fn move_bud(&mut self, moving: Point, delta_time: f32) {
        if moving.x != 0 || moving.y != 0 {
            self.bud_data.borrow_mut().speed -= 1;
            self.position += moving;
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
    fn start(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) -> bool {
        self.active = true;
        println!("Start Turn!");

        self.apply_effects();
        true
    }
    fn end(&mut self) -> bool {
        self.active = false;
        println!("End Turn!");
        self.bud_data.borrow_mut().reset();
        return true;
    }
    fn update(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
        // level_info: &mut LevelInfo<'g>,
    ) -> bool {
        if self.active {
            self.decide_move(gi, _delta_time);
        }
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
        todo!();
        // (
        //     Rect::from_center(camera.point_to_camera(&mut self.position.clone()), 10, 10),
        //     "",
        // )
    }

    fn hover_action(&mut self, input: &mut Self::Input) {
        input.load_menu(MenuStateEnum::Bud(None, Some(Rc::clone(&self.bud_data))));
        println!("Hovered");
    }

    fn action(&mut self, input: &mut Self::Input) {
        input.load_menu(MenuStateEnum::Bud(Some(Rc::clone(&self.bud_data)), None));
        println!("Pressed");
    }

    fn in_bounds(&self, mouse_x: i32, mouse_y: i32, camera: Option<&Camera>) -> bool {
        let mut rect = Rect::from_center(self.position, 10, 10);
        // let mut center = self.position.clone();
        if let Some(camera) = camera {
            // camera.point_to_camera(&mut center);
            camera.rect_to_camera(&mut rect);
        }
        // let rect = Rect::from_center(center, 10, 10);
        (mouse_x >= rect.left() && mouse_x <= rect.right())
            && (mouse_y >= rect.top() && mouse_y <= rect.bottom())
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
    pub fn reset(&mut self) {
        self.speed = self.initial.max_speed;
    }

    fn default(initial: Rc<InitialBudData<'g>>) -> BudData<'g> {
        BudData {
            initial,
            selected: false,
            health: 10,
            speed: 3,
        }
    }
}

pub struct InitialBudData<'g> {
    texture: Rc<Texture<'g>>,
    max_health: u16,
    max_speed: u16,
    // traits:
}

impl<'g> InitialBudData<'g> {
    pub fn default(texture: Rc<Texture<'g>>) -> InitialBudData {
        InitialBudData {
            texture,
            max_health: 10,
            max_speed: 3,
        }
    }
}
