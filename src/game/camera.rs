use std::cmp::{max, min};

use sdl2::{
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::vector2d::Vector2d;

pub struct Camera {
    pub position: Vector2d,
    pub window_size: (u32, u32),
    pub scale: i32,
    pub blocks: (u32, u32),
}

impl Camera {
    pub fn default() -> Camera {
        Camera {
            position: Vector2d::default(),
            window_size: (0, 0),
            scale: 2,
            blocks: (100, 100),
        }
    }
    pub fn window_scale(&self) -> f32 {
        (self.window_size.0 as f32 / self.blocks.0 as f32)
            .max(self.window_size.1 as f32 / self.blocks.1 as f32)
    }
    pub fn blocks(&mut self) -> (u32, u32) {
        let window_scale = self.window_scale();
        return (
            (self.window_size.0 as f32 / window_scale) as u32,
            (self.window_size.1 as f32 / window_scale) as u32,
        );
    }
    pub fn reset(&mut self) {
        self.position = Vector2d::default();
    }
    pub fn set_window(&mut self, canvas: &mut Canvas<Window>) {
        self.window_size = canvas.window().size();
        canvas.window_mut().set_size(
            self.blocks().0 * self.window_scale() as u32,
            self.blocks().1 * self.window_scale() as u32,
        );
    }
    pub fn set_camera(&mut self, delta_time: f32, player_position: &Vector2d, scale: i32) {
        self.position = player_position.clone();
        self.scale = scale;
        return;
        let add_x = 0.0015 * delta_time * (self.position.x - player_position.x);
        let add_y = 0.0015 * delta_time * (self.position.y - player_position.y);
        if (self.position.x - player_position.x).abs() >= 5.0 {
            self.position.x -= add_x;
        }
        if (self.position.y - player_position.y).abs() >= 5.0 {
            self.position.y -= add_y;
        }
    }
    pub fn set_camera_mouse(&mut self, delta_time: f32, mouse_pos: &Vector2d) {
        let (width, height) = self.window_size;
        let width = width as f32 / 2.0;
        let height = height as f32 / 2.0;
        let factor = 0.0015 * delta_time * 100.0 * self.scale as f32 / 4.0;

        if self.in_bounds(mouse_pos) {
            if mouse_pos.y > 1.9 * height {
                self.position.y += factor * (mouse_pos.y.abs() - height.abs()) / height;
            } else if mouse_pos.y < 0.1 * height {
                self.position.y += factor * (mouse_pos.y.abs() - height.abs()) / height;
            }
            if mouse_pos.x > 1.9 * width {
                self.position.x += factor * (mouse_pos.x.abs() - width.abs()) / width;
            } else if mouse_pos.x < 0.1 * width {
                self.position.x += factor * (mouse_pos.x.abs() - width.abs()) / width;
            }
        }
    }
    pub fn in_bounds(&self, pos: &Vector2d) -> bool {
        let (width, height) = self.window_size;
        pos.x < width as f32 - 1.0 && pos.x > 0.0 && pos.y < height as f32 - 1.0 && pos.y > 0.0
    }
    pub fn rect_to_camera(&self, rect: &mut Rect) {
        let center_x = rect.center().x();
        let center_y = rect.center().y();
        let mut x = center_x as f32 - self.position.x;
        x /= self.scale as f32 / self.window_scale();
        let mut y = center_y as f32 - self.position.y;
        y /= self.scale as f32 / self.window_scale();

        rect.w *= self.window_scale() as i32;
        rect.h *= self.window_scale() as i32;
        rect.w /= (self.scale);
        rect.h /= (self.scale);
        // rect.x -= (self.position.x as i32);
        // rect.x /= (self.scale as f32);
        // rect.y -= (self.position.y as i32);
        // rect.y /= (self.scale as f32);
        rect.center_on(Point::new(x as i32, y as i32));
        // rect.x = x as i32;
        // rect.y = y as i32;
    }
    pub fn rect_scaled_to_camera(&self, rect: &mut Rect) {
        rect.x *= self.window_size.0 as i32 / 3 / 3;
        rect.y *= self.window_size.1 as i32 / 3 / 3;
        rect.w *= self.window_size.0 as i32 / 3 / 3;
        rect.h *= self.window_size.1 as i32 / 3 / 3;
    }
    pub fn vector2ds_to_camera(&self, vector2ds: &mut Vec<Vector2d>) {
        for vector2d in vector2ds.iter_mut() {
            self.vector2d_to_camera(vector2d);
        }
    }
    pub fn vector2d_to_camera(&self, vector2d: &mut Vector2d) {
        // println!("Initial: {}, {}", vector2d.x, vector2d.y);

        let mut x = vector2d.x - self.position.x;
        x /= self.scale as f32 / self.window_scale();
        let mut y = vector2d.y - self.position.y;
        y /= self.scale as f32 / self.window_scale();
        *vector2d = Vector2d::new(x, y);
        // println!("Later: {}, {}", vector2d.x, vector2d.y);
    }
    pub fn points_to_camera(&self, points: &mut Vec<Point>) {
        for point in points.iter_mut() {
            self.point_to_camera(point);
        }
    }
    pub fn point_to_camera(&self, point: &mut Point) {
        let mut x = point.x as f32 - self.position.x;
        x /= self.scale as f32 / self.window_scale();
        let mut y = point.y as f32 - self.position.y;
        y /= self.scale as f32 / self.window_scale();
        *point = Point::new(x as i32, y as i32);
    }
    pub fn get_distance(&self, vector2d: &Vector2d) -> f32 {
        f32::powf(
            f32::powf(self.position.x as f32 - vector2d.x, 2.0)
                + f32::powf(self.position.y as f32 - vector2d.y, 2.0),
            0.5,
        )
    }
}
