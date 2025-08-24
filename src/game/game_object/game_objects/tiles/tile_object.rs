use std::{cell::RefCell, rc::Rc};

use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    sys::SDL_atan2,
    video::Window,
};

use crate::{
    game::{
        button::NoButton, camera::Camera, collision_system::collisions::Colliding,
        effect_system::effects::Tile, game_info::GameInfo, game_object::GameObject,
    },
    vector2d::Vector2d,
};

pub struct TileObject<'g> {
    position: Rc<RefCell<Vector2d>>,
    size: Vector2d,
    // texture: &'g Texture<'g>,
    texture: Rc<RefCell<Texture<'g>>>,
    id: &'g str,
    src: Rect,
}

impl<'g> TileObject<'g> {
    // vector2d* p, vector2d* s, SDL_Texture* texture, bool hasCol, vector2d srcPos
    pub fn new(
        position: Vector2d,
        size: Vector2d,
        // texture: &'g Texture,
        texture: Rc<RefCell<Texture<'g>>>,
        _has_col: bool,
        src_pos: Vector2d,
    ) -> Self {
        let src = Rect::new(src_pos.x as i32, src_pos.y as i32, 16, 16);
        let mut p = position.clone();
        let mut s = size.clone();
        s.x *= 1.0;
        s.y *= 1.0;
        let _pos = Rc::new(RefCell::new(p));
        let __pos = Rc::clone(&_pos);
        Self {
            position: _pos,
            size: s,
            texture: Rc::clone(&texture),
            id: "TileObject",
            src,
        }
    }
}

impl<'g> GameObject<'g> for TileObject<'g> {
    fn get_position(&self) -> Vector2d {
        self.position.borrow().clone()
    }

    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        // unsafe { CANVAS.set_draw_color(Color::RGBA(139, 210, 241, 255)) };
        canvas.set_draw_color(Color::RGBA(139, 210, 241, 255));
        println!(
            "pos: {}, {}",
            self.position.borrow().x,
            self.position.borrow().y
        );

        let mut some_rect = Rect::from_center(
            Point::new(
                self.position.borrow().x as i32,
                self.position.borrow().y as i32,
            ),
            self.size.x as u32,
            self.size.y as u32,
        );
        //     0,   //self.position.borrow().x as i32,
        //     0,   //self.position.borrow().y as i32,
        //     100, //(self.size.y * 400.0) as u32,
        //     100, //(self.size.y * 400.0) as u32,
        // );
        camera.rect_to_camera(&mut some_rect);
        canvas.copy_ex(
            &self.texture.borrow_mut(),
            self.src,
            some_rect,
            0.0,
            None,
            false,
            false,
        );
    }

    fn get_draw_values(&self) -> (Vector2d, Vector2d) {
        todo!()
    }
    // fn update(&mut self, delta_time: f32) -> bool {}
}

impl<'r> Colliding<'r> for TileObject<'r> {
    fn get_collider(&self) -> sdl2::rect::Point {
        return Point::new(
            self.position.borrow().x as i32,
            self.position.borrow().y as i32,
        );
    }

    fn on_effected(
        &mut self,
        effect: Box<dyn crate::game::effect_system::effects::Effect<'r> + 'r>,
        others: Vec<Rc<RefCell<dyn Colliding<'r> + 'r>>>,
    ) {
        todo!()
    }
}

impl<'g> NoButton for TileObject<'g> {}
