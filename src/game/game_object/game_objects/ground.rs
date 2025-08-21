use libnoise::{Generator, Generator1D, Simplex, Source, Worley};
use std::rc::Rc;

use sdl2::{
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    pixels::{Color, PixelFormat, PixelFormatEnum},
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    sys::{SDL_AllocFormat, SDL_MapRGB},
    video::{Window, WindowContext},
    EventPump,
};

use crate::{
    game::{
        button::{Button, NoButton},
        camera::Camera,
        collision_system::collisions::Collisions,
        game_info::GameInfo,
        game_object::{game_objects::GameObjectEnum, GameObject, SuperGameObject},
        game_state::StateInfo,
        input::Input,
    },
    vector2d::Vector2d,
};

pub struct Ground<'s> {
    position: Vector2d,
    tex: Texture<'s>,
}

impl<'s> Ground<'s> {
    pub fn new(position: Vector2d, mut tex: Texture<'s>) -> Self {
        Self { position, tex }
    }
}

impl<'g> GameObject<'g> for Ground<'g> {
    fn get_position(&self) -> Vector2d {
        self.position.clone()
    }

    fn get_draw_values(&self) -> (Vector2d, Vector2d) {
        (self.position.clone(), Vector2d::new(1.0, 1.0))
    }

    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        // let (position, size) = self.get_draw_values();
        for i in 0..100 {
            for j in 0..100 {
                let mut some_rect = Rect::from_center(Point::new(i, j), 16, 16);
                camera.rect_to_camera(&mut some_rect);
                canvas.copy_ex(
                    &self.tex, None, some_rect, 0.0, None,
                    // Point::new(self.position.x as i32 + 10, self.position.y as i32 + 10),
                    // some_rect.top_left() + Point::new(0, 0),
                    false, false,
                );
            }
        }
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
}

impl<'g> NoButton for Ground<'g> {}
