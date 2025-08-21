use core::fmt;
use std::fmt::write;
use std::path::Display;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::game::button::{Button, NoButton};
use crate::game::camera::Camera;
use crate::game::game_info::GameInfo;
use crate::game::game_object::game_objects::GameObjectEnum;
use crate::game::game_state::StateInfo;
use crate::game::menu::menu_state::menu_states::MenuStateHandler;
use crate::vector2d::Vector2d;

use super::collision_system::collisions::Collisions;
use super::input::Input;

pub mod game_objects;

pub trait SuperGameObject<'g>: GameObject<'g> + Button<'g> {}

impl<'g, T> SuperGameObject<'g> for T where T: GameObject<'g> + Button<'g> {}

impl<'g, T> Button<'g> for T
where
    T: GameObject<'g> + NoButton,
{
    fn get_pressed(&self) -> (bool, bool) {
        (false, false)
    }

    fn set_hovered(&mut self, hovered: bool) {}

    fn set_pressed(&mut self, pressed: bool) {}

    fn get_draw_values(&self) -> (Rect, &str) {
        (Rect::new(0, 0, 0, 0), &"")
    }

    fn action(&mut self, input: &mut Self::Input) {}

    type Input = MenuStateHandler<'g>;
}

// impl<'g, T> Button<'g> for T
// where
//     T: GameObject<'g> - NoButton,
// {
//     fn get_pressed(&self) -> (bool, bool) {
//         (false, false)
//     }

//     fn set_hovered(&mut self, hovered: bool) {}

//     fn set_pressed(&mut self, pressed: bool) {}

//     fn get_draw_values(&self) -> (Rect, &str) {
//         (Rect::new(0, 0, 0, 0), &"")
//     }

//     fn action(&mut self, input: &mut Self::Input) {}

//     type Input = GameObjectEnum;
// }

pub trait GameObject<'g> {
    // fn default() -> Self;
    // fn new(position: &Vector2d, size: &Vector2d, texture: &Texture, hasCol: bool) -> Self;
    fn get_position(&self) -> Vector2d;
    // fn get_size(&self) -> Vector2d;
    fn get_draw_values(&self) -> (Vector2d, Vector2d);
    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        let (position, size) = self.get_draw_values();
        canvas.set_draw_color(Color::RGBA(139, 210, 241, 255));
        let mut some_rect = Rect::new(
            position.x as i32,
            position.y as i32,
            size.y as u32,
            size.y as u32,
        );
        camera.rect_to_camera(&mut some_rect);
        canvas.draw_rect(some_rect);
    }
    fn start(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
        msh: &mut MenuStateHandler<'g>,
    ) -> bool {
        true
    }
    fn end(&mut self) -> bool {
        true
    }
    fn update(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) -> bool {
        true
    }

    // fn on_hit(&mut self, other_obj: Box<dyn GameObject>, info: HitInfo);

    // fn hit_target(&mut self, other_obj: Box<dyn GameObject>, info: HitInfo);

    // fn create_twin(&self) {
    //     println!("Twin Made");
    // }
    // fn get_col(&self) -> (bool, Collider);
    // fn on_scene_add(&self) {
    //     let (has_col, col) = self.get_col();
    //     if (has_col) {
    //         Collisions::AddCollider(col);
    //     }
    // }
}
