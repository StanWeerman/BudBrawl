use std::{
    cell::{Cell, RefCell},
    ops::Deref,
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
        effect_system::effects::{aura_effect::AuraEffect, self_effect::DamageEffect, Effect},
        game_info::GameInfo,
        game_object::{
            game_objects::{
                bud::bud_data::{BudData, InitialBudData},
                GameObjectEnum,
            },
            GameObject, SuperGameObject,
        },
        game_state::{game_states::select_state::NameGenerator, StateInfo},
        input::Input,
        menu::menu_state::menu_states::{BudEnum, MenuStateEnum, MenuStateHandler},
    },
    vector2d::Vector2d,
};

pub mod bud_data;
pub mod weapon;

pub struct Bud<'g> {
    position: Point,
    bud_data: Rc<RefCell<BudData<'g>>>,
    hovered: bool,
    pressed: bool,
    effects: Vec<Box<dyn Effect<'g> + 'g>>,
    moved: [bool; 4],
    direction: Direction,
    active: bool,
    dead: bool,
}
impl<'g> Bud<'g> {
    pub fn new(position: Point, initial_blud_data: InitialBudData<'g>) -> Self {
        Self {
            position,
            bud_data: Rc::new(RefCell::new(BudData::default(initial_blud_data))),
            hovered: false,
            pressed: false,
            effects: vec![],
            moved: [false, false, false, false],
            active: false,
            direction: Direction::Down,
            dead: false,
        }
    }

    pub fn decide_move(
        &mut self,
        gi: &mut GameInfo<'g>,
        collisions: &mut Collisions,
        delta_time: f32,
    ) {
        if self.bud_data.borrow().speed > 0 {
            let mut moving = Point::new(0, 0);
            if gi.input.is_pressed(Keycode::W) && !self.moved[0] {
                self.moved[0] = true;
                moving.x = 0;
                moving.y = -1;
                self.direction = Direction::Up;
            } else if gi.input.is_released(Keycode::W) {
                self.moved[0] = false;
            }
            if gi.input.is_pressed(Keycode::S) && !self.moved[1] {
                self.moved[1] = true;
                moving.x = 0;
                moving.y = 1;
                self.direction = Direction::Down;
            } else if gi.input.is_released(Keycode::S) {
                self.moved[1] = false;
            }
            if gi.input.is_pressed(Keycode::A) && !self.moved[2] {
                self.moved[2] = true;
                moving.x = -1;
                moving.y = 0;
                self.direction = Direction::Left;
            } else if gi.input.is_released(Keycode::A) {
                self.moved[2] = false;
            }
            if gi.input.is_pressed(Keycode::D) && !self.moved[3] {
                self.moved[3] = true;
                moving.x = 1;
                moving.y = 0;
                self.direction = Direction::Right;
            } else if gi.input.is_released(Keycode::D) {
                self.moved[3] = false;
            }
            self.move_bud(moving, collisions, delta_time);
        }
    }
    fn died(&mut self) {
        self.dead = true;
    }
    fn attack(&mut self, collisions: &mut Collisions) {
        let attack_tile = self.position + self.direction.get_point();
        if (collisions.impact_tile(attack_tile, Box::new(DamageEffect::new(10)))) {}
    }

    pub fn move_bud(&mut self, moving: Point, collisions: &mut Collisions, delta_time: f32) {
        if collisions.check_tile(self.position + moving) {
            return;
        }
        if moving.x != 0 || moving.y != 0 {
            self.bud_data.borrow_mut().speed -= 1;
            self.position += moving;
        }
    }
    pub fn add_effect(
        &mut self,
        mut eff: Box<dyn Effect<'g> + 'g>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        println!("EFFECTED");
        eff.apply(Rc::clone(&self.bud_data), others);
        self.effects.push(eff);
    }
    pub fn apply_effects(&mut self, others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>) {
        for i in 0..3 {
            if let Some(new_effect) = self.bud_data.borrow_mut().initial.effects[i].clone() {
                new_effect
                    .borrow_mut()
                    .apply(Rc::clone(&self.bud_data), others.clone());
            }
        }
        self.effects
            .iter_mut()
            .filter(|eff| eff.is_active())
            .for_each(|eff| eff.apply(Rc::clone(&self.bud_data), others.clone()));
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
        if self.bud_data.borrow().health == 0 {
            return;
        }
        // let (position, size) = self.get_draw_values();
        canvas.set_draw_color(Color::RGBA(139, 210, 241, 255));
        let mut some_rect = Rect::from_center(self.position, 16, 21);
        camera.rect_to_camera(&mut some_rect);
        // some_rect.x += (1.0 * camera.window_scale() / 2 as f32) as i32;
        some_rect.y -= (8.0 * camera.window_scale() / 2 as f32) as i32;

        let tex_src = match self.direction {
            Direction::Down => Rect::new(0, 0, 16, 21),
            Direction::Right => Rect::new(16, 0, 16, 21),
            Direction::Up => Rect::new(0, 21, 16, 21),
            Direction::Left => Rect::new(16, 21, 16, 21),
        };

        canvas.copy_ex(
            &self.bud_data.borrow().initial.texture,
            tex_src,
            some_rect,
            0.0,
            None,
            // Point::new(self.position.x as i32 + 10, self.position.y as i32 + 10),
            // some_rect.top_left() + Point::new(0, 0),
            false,
            false,
        );
    }
    fn start(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions<'g>,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
        msh: &mut MenuStateHandler<'g>,
    ) -> bool {
        if !self.dead && self.bud_data.borrow().health == 0 {
            self.dead = true;
            collisions.remove(self.position);
        }
        if self.dead {
            return false;
        }
        self.active = true;
        self.apply_effects(collisions.colliders.clone());

        println!(
            "Start Turn! This is bud {}, with {} health.",
            self.bud_data.borrow().initial.name,
            self.bud_data.borrow().health
        );

        msh.load_menu(MenuStateEnum::Bud(BudEnum::LeftBud(Some(Rc::clone(
            &self.bud_data,
        )))));
        true
    }
    fn end(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
        msh: &mut MenuStateHandler<'g>,
    ) -> bool {
        if !self.dead && self.bud_data.borrow().health == 0 {
            self.dead = true;
            collisions.remove(self.position);
        }
        if self.dead {
            return false;
        }
        self.active = false;
        self.attack(collisions);
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
            self.decide_move(gi, collisions, _delta_time);
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
        input.load_menu(MenuStateEnum::Bud(BudEnum::RightBud(Some(Rc::clone(
            &self.bud_data,
        )))));
    }

    fn action(&mut self, input: &mut Self::Input) {
        // input.load_menu(MenuStateEnum::Bud(BudEnum::LeftBud(Some(Rc::clone(
        //     &self.bud_data,
        // )))));
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

impl<'g> Colliding<'g> for Bud<'g> {
    fn on_effected(
        &mut self,
        effect: Box<dyn Effect<'g> + 'g>,
        others: Vec<Rc<RefCell<dyn Colliding<'g> + 'g>>>,
    ) {
        self.add_effect(effect, others);
    }

    fn get_collider(&self) -> Point {
        self.position
    }
}

pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Direction {
    fn get_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Right => Point::new(1, 0),
            Direction::Left => Point::new(-1, 0),
            Direction::Down => Point::new(0, 1),
        }
    }
}
