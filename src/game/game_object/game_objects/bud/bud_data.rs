use std::{cell::RefCell, rc::Rc};

use sdl2::{
    gfx::primitives::DrawRenderer,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
};

use crate::game::{
    camera::Camera,
    effect_system::effects::{aura_effect::AuraEffect, self_effect::DamageEffect, Effect},
    game_object::game_objects::bud::weapon::{Weapon, WeaponInfo},
    game_state::game_states::select_state::NameGenerator,
};

pub struct BudData<'g> {
    pub initial: InitialBudData<'g>,
    pub selected: bool,
    pub max_health: u16,
    pub health: u16,
    pub speed: u16,
    pub damage: u16,
}

impl<'g> BudData<'g> {
    pub fn remove_health(&mut self, dmg: u16) {
        println!("Dealt {} dmg", dmg);
        if self.health >= dmg {
            self.health -= dmg;
        } else if self.health < dmg {
            self.health = 0;
        }
    }
    pub fn add_health(&mut self, health: u16) {
        println!("Healed {} dmg", health);
        if self.health + health > self.max_health {
            self.health = self.max_health;
        } else {
            self.health += health;
        }
    }
    pub fn select(&mut self) {
        self.selected = true;
    }
    pub fn unselect(&mut self) {
        self.selected = false;
    }
    pub fn reset(&mut self) {
        self.speed = self.initial.max_speed;
        self.damage = 0;
        self.max_health = self.initial.max_health;
    }

    pub fn alive(&self) -> bool {
        self.health > 0
    }

    pub fn draw_bud_data(&self, index: i32, canvas: &mut Canvas<Window>, camera: &Camera) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        let mut point = Point::new(20 * index as i32 + 1, 80);
        camera.ui_point_to_camera(&mut point);
        canvas.string(
            point.x as i16,
            point.y as i16,
            &format!("Health: {}/{}", self.health, self.initial.max_health),
            sdl2::pixels::Color::RGB(0, 0, 0),
        );
        let mut point = Point::new(20 * index as i32 + 1, 82);
        camera.ui_point_to_camera(&mut point);
        canvas.string(
            point.x as i16,
            point.y as i16,
            &format!("Speed: {}/{}", self.speed, self.initial.max_speed),
            sdl2::pixels::Color::RGB(0, 0, 0),
        );
    }

    pub fn default(initial: InitialBudData<'g>) -> BudData<'g> {
        BudData {
            initial,
            selected: false,
            max_health: 10,
            health: 10,
            speed: 3,
            damage: 0,
        }
    }
}

#[derive(Clone)]
pub struct InitialBudData<'g> {
    pub texture: Rc<Texture<'g>>,
    pub max_health: u16,
    pub max_speed: u16,
    pub index: u8,
    pub team: u8,
    pub rounds: u64,
    pub effects: [Option<Box<dyn Effect<'g>>>; 3],
    pub effect_textures: [Option<Rc<Texture<'g>>>; 3],
    pub name: String,
    pub weapon_info: WeaponInfo<'g>,
}

impl<'g> InitialBudData<'g> {
    pub fn default(
        texture: Rc<Texture<'g>>,
        team: u8,
        index: u8,
        name_generator: &NameGenerator,
        weapon_info: WeaponInfo<'g>,
    ) -> InitialBudData<'g> {
        let name = name_generator.selectRandName();

        InitialBudData {
            texture,
            max_health: 10,
            max_speed: 3,
            index,
            team,
            rounds: 0,
            effect_textures: [None, None, None],
            effects: [None, None, None],
            //Some(Rc::new(RefCell::new(AuraEffect::new(Box::new(DamageEffect::new(10))))))
            name,
            weapon_info,
        }
    }
    pub fn add_effect(&mut self, new_effect: Box<dyn Effect<'g>>, tex: Option<Rc<Texture<'g>>>) {
        for (i, effect) in self.effects.iter_mut().enumerate() {
            if effect.is_none() {
                *effect = Some(new_effect);
                if tex.is_some() {
                    self.effect_textures[i] = Some(Rc::clone(&tex.unwrap()));
                }
                break;
            }
        }
    }
    pub fn clear_effects(&mut self) {
        self.effects = [None, None, None];
        self.effect_textures = [None, None, None];
    }
    pub fn change_weapon(
        &mut self,
        weapon_info: WeaponInfo<'g>,
        // tex: Option<Rc<Texture<'g>>>,
    ) {
        self.weapon_info = weapon_info;
    }

    pub fn new_round(&mut self, index: u8) {
        self.rounds += 1;
        self.index = index;
    }

    pub fn draw_initial_bud_data(&self, index: i32, canvas: &mut Canvas<Window>, camera: &Camera) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        let mut rect = Rect::new(20 * index as i32, 20, 20, 60);
        camera.ui_rect_to_camera(&mut rect);
        rect.x += 1;
        rect.y += 1;
        rect.w -= 2;
        rect.h -= 2;
        canvas.draw_rect(rect);

        let mut point = Point::new(20 * index as i32 + 1, 21);
        camera.ui_point_to_camera(&mut point);
        canvas.string(
            point.x as i16,
            point.y as i16,
            &self.name,
            sdl2::pixels::Color::RGB(0, 0, 0),
        );

        let mut point = Point::new(20 * index as i32 + 1, 24);
        camera.ui_point_to_camera(&mut point);
        canvas.string(
            point.x as i16,
            point.y as i16,
            &format!("Team {} | Bud {}", self.team + 1, self.index + 1),
            sdl2::pixels::Color::RGB(0, 0, 0),
        );

        let mut point = Point::new(20 * index as i32 + 1, 27);
        camera.ui_point_to_camera(&mut point);
        canvas.string(
            point.x as i16,
            point.y as i16,
            &format!("Lived for {} Rounds", self.rounds),
            sdl2::pixels::Color::RGB(0, 0, 0),
        );

        let mut weapon_rect = self.weapon_info.weapon_rect.clone();
        camera.ui_rect_to_camera(&mut weapon_rect);
        weapon_rect.x *= index;
        canvas.copy_ex(
            &self.weapon_info.weapon_texture,
            None,
            weapon_rect,
            0.0,
            None,
            false,
            false,
        );

        for (i, effect_texture) in self.effect_textures.iter().enumerate() {
            let mut rect = Rect::new(20 * index as i32 + 6 * i as i32, 40, 6, 6);
            camera.ui_rect_to_camera(&mut rect);
            rect.x += 2;
            if let Some(effect_texture) = effect_texture {
                canvas.copy_ex(&effect_texture, None, rect, 0.0, None, false, false);
            } else if self.effects[i].is_some() {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas.fill_rect(rect);
            }
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.draw_rect(rect);
            rect.x -= 1;
            rect.y -= 1;
            rect.w += 2;
            rect.h += 2;
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.draw_rect(rect);
        }
    }
}
