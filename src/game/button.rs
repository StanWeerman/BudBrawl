use std::{cell::RefCell, rc::Rc};

use sdl2::{
    gfx::primitives::DrawRenderer,
    mouse::MouseState,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture},
    video::Window,
    EventPump,
};

use crate::game::{camera::Camera, game_info::GameInfo, input::Input};

pub trait NoButton {}

pub trait Button<'b> {
    fn get_pressed(&self) -> (bool, bool);
    fn set_hovered(&mut self, hovered: bool);
    fn set_pressed(&mut self, pressed: bool);
    fn get_draw_values(&self) -> (Rect, &str);
    fn hover_action(&mut self, input: &mut Self::Input) {}
    fn action(&mut self, input: &mut Self::Input);
    fn draw(&self, canvas: &mut Canvas<Window>, camera: &Camera) {
        let (pressed, hovered) = self.get_pressed();
        let (mut rect, text) = self.get_draw_values();
        camera.ui_rect_to_camera(&mut rect);
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(rect).unwrap();
        if pressed {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
        } else if hovered {
            canvas.set_draw_color(Color::RGB(0, 0, 255));
        } else {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas.draw_rect(rect).unwrap();
        canvas.string(
            rect.x as i16 + 10,
            rect.y as i16 + 10,
            &text,
            Color::RGB(0, 0, 0),
        );
    }
    type Input;

    fn press(
        &mut self,
        mouse_state: &MouseState,
        button_input: &mut Self::Input,
        camera: Option<&Camera>,
    ) -> (bool, bool) {
        // camera.rect_scaled_to_camera(&mut self.rect);
        let (hovered, pressed) = self.get_pressed();
        // self.set_hovered(hovered);
        let m = mouse_state;
        let bounds = self.in_bounds(m.x(), m.y(), camera);
        if bounds {
            self.set_hovered(true);
            if hovered == false {
                self.hover_action(button_input);
            }
        } else {
            self.set_hovered(false);
        }
        if m.left() && !pressed && bounds {
            self.set_pressed(true);
        } else if !m.left() && pressed && bounds {
            self.set_pressed(false);
            // (&self.action)(input);
            self.action(button_input);
        } else if !m.left() && pressed {
            self.set_pressed(false);
        }
        self.get_pressed()
    }
    fn in_bounds(&self, mouse_x: i32, mouse_y: i32, camera: Option<&Camera>) -> bool {
        let (mut rect, text) = self.get_draw_values();
        if let Some(camera) = camera {
            camera.ui_rect_to_camera(&mut rect);
        }
        (mouse_x >= rect.left() && mouse_x <= rect.right())
            && (mouse_y >= rect.top() && mouse_y <= rect.bottom())
    }
}

pub struct MenuButton<T> {
    pressed: bool,
    hovered: bool,
    rect: Rect,
    text: &'static str,
    pub action: Box<dyn Fn(&mut T)>,
}
impl<'t, T> MenuButton<T> {
    pub fn new(
        rect: Rect,
        text: &'static str,
        action: Box<dyn Fn(&mut T)>,
        // gi: &GameInfo<'t>,
    ) -> Self {
        // let mut tex = None;
        // if let Some(tex_) = gi.textures.get(text) {
        //     tex = Some(Rc::clone(tex_));
        // }
        Self {
            // tex,
            pressed: false,
            hovered: false,
            rect,
            text,
            action,
        }
    }
}

impl<'b, T> Button<'b> for MenuButton<T> {
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
        (self.rect, self.text)
    }

    fn action(&mut self, input: &mut Self::Input) {
        (&self.action)(input);
    }

    type Input = T;
}

pub struct HoverMenuButton<'t, T> {
    pressed: bool,
    hovered: bool,
    rect: Rect,
    text: &'static str,
    texture: Rc<Texture<'t>>,
    pub action: Box<dyn Fn(&mut T)>,
    pub hover_action: Box<dyn Fn(&mut T)>,
}
impl<'t, T> HoverMenuButton<'t, T> {
    pub fn new(
        rect: Rect,
        text: &'static str,
        texture: Rc<Texture<'t>>,
        action: Box<dyn Fn(&mut T)>,
        hover_action: Box<dyn Fn(&mut T)>,
        // gi: &GameInfo<'t>,
    ) -> Self {
        // let mut tex = None;
        // if let Some(tex_) = gi.textures.get(text) {
        //     tex = Some(Rc::clone(tex_));
        // }
        Self {
            // tex,
            pressed: false,
            hovered: false,
            rect,
            text,
            texture,
            action,
            hover_action,
        }
    }
}

impl<'b, T> Button<'b> for HoverMenuButton<'b, T> {
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
        (self.rect, self.text)
    }

    fn hover_action(&mut self, input: &mut Self::Input) {
        (&self.hover_action)(input);
    }

    fn action(&mut self, input: &mut Self::Input) {
        (&self.action)(input);
    }

    type Input = T;
}
