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
        button::{Button, NoButton},
        camera::Camera,
        collision_system::collisions::{Colliding, Collisions, Side},
        game_info::GameInfo,
        game_object::{game_objects::GameObjectEnum, GameObject, SuperGameObject},
        game_state::StateInfo,
        input::Input,
        menu::menu_state::menu_states::{MenuStateEnum, MenuStateHandler},
    },
    vector2d::Vector2d,
};

pub struct Shell<'s> {
    position: Vector2d,
    shell_data: Rc<RefCell<ShellData<'s>>>,
}

impl<'s> Shell<'s> {
    pub fn new(
        position: Vector2d,
        initial_shell_data: Rc<InitialShellData<'s>>,
        selected: Rc<Cell<bool>>,
    ) -> Self {
        Self {
            position,
            shell_data: Rc::new(RefCell::new(ShellData::default(
                initial_shell_data,
                selected,
            ))),
        }
    }
    pub fn move_shell(&mut self, delta_time: f32) -> Point {
        let moving = self.shell_data.borrow_mut().moving;
        if moving > 0.0 {
            {
                let mut shell_data = self.shell_data.borrow_mut();
                // println!("Moving: {}", self.moving);
                shell_data.moving -= delta_time;
                println!("{}", shell_data.speed);
                // self.speed += (delta_time / 1000.0) * self.new_speed;
                shell_data.speed -= delta_time / 10.0;
                shell_data.speed = f32::max(shell_data.speed, 0.0);
                self.position.x +=
                    shell_data.angle.to_radians().cos() * (delta_time / 1000.0) * shell_data.speed;
                self.position.y +=
                    shell_data.angle.to_radians().sin() * (delta_time / 1000.0) * shell_data.speed;
            }
        }
        return Point::new(
            (self.position.x + self.shell_data.borrow().initial.size.y / 2.0) as i32,
            (self.position.y + self.shell_data.borrow().initial.size.y / 2.0) as i32,
        );
    }
}

impl<'g> GameObject<'g> for Shell<'g> {
    fn get_position(&self) -> Vector2d {
        self.position.clone()
    }

    fn get_draw_values(&self) -> (Vector2d, Vector2d) {
        (
            self.position.clone(),
            self.shell_data.borrow().initial.size.clone(),
        )
    }

    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        // let (position, size) = self.get_draw_values();
        let mut some_rect = Rect::from_center(
            Point::new(self.position.x as i32, self.position.y as i32),
            self.shell_data.borrow().initial.size.x as u32,
            self.shell_data.borrow().initial.size.y as u32,
        );
        camera.rect_to_camera(&mut some_rect);
        canvas.copy_ex(
            &self.shell_data.borrow().initial.texture,
            None,
            some_rect,
            self.shell_data.borrow().angle as f64,
            None,
            false,
            false,
        );

        if self.shell_data.borrow().selected.get() {
            let mut center = self.position.clone();
            camera.vector2d_to_camera(&mut center);
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_point(Point::new(center.x as i32, center.y as i32));
        }
    }
    fn update(
        &mut self,
        delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) -> bool {
        self.move_shell(delta_time);
        // return self.shell_data.borrow().speed != 0.0;
        true
    }
}

impl<'g> Colliding for Shell<'g> {
    fn has_collided(&mut self, other: &dyn Colliding) {
        todo!()
    }

    fn get_collider(&self) -> Rc<RefCell<Vec<Point>>> {
        // Rc::clone(&self.points)
        Rc::new(RefCell::new(vec![Point::new(
            self.position.x as i32,
            self.position.y as i32,
        )]))
    }
}

pub struct InitialShellData<'s> {
    max_speed: f32,
    max_angle: f32,
    texture: Rc<Texture<'s>>,
    size: Vector2d,
}

pub struct ShellData<'s> {
    initial: Rc<InitialShellData<'s>>,
    angle: f32,
    speed: f32,
    acceleration: f32,
    moving: f32,
    selected: Rc<Cell<bool>>,
}

impl<'s> ShellData<'s> {
    pub fn default(initial: Rc<InitialShellData<'s>>, selected: Rc<Cell<bool>>) -> ShellData<'s> {
        ShellData {
            initial,
            angle: 90.0,
            speed: 790.0,
            acceleration: 0.0,
            moving: 100000.0,
            selected,
        }
    }
}

const FLETCHER_HEIGHT: i32 = 114;
const FLETCHER_WIDTH: i32 = 12;

impl<'s> InitialShellData<'s> {
    pub fn default(texture: Rc<Texture<'s>>) -> InitialShellData {
        InitialShellData {
            max_speed: 0.0,
            max_angle: 0.0,
            texture: texture,
            size: Vector2d::new(0.68 as f32, 0.127 as f32),
        }
    }
}

impl<'g> NoButton for Shell<'g> {}

// impl<'g> Button<'g> for Ship<'g> {
//     fn get_pressed(&self) -> (bool, bool) {
//         (self.hovered, self.pressed)
//     }

//     fn set_hovered(&mut self, hovered: bool) {
//         self.hovered = hovered;
//     }

//     fn set_pressed(&mut self, pressed: bool) {
//         self.pressed = pressed;
//     }

//     fn get_draw_values(&self) -> (Rect, &str) {
//         todo!()
//     }

//     fn action(&mut self, input: &mut Self::Input) {
//         input.load_menu(MenuStateEnum::Ship(Some(Rc::clone(&self.shell_data))));
//     }

//     fn in_bounds(&self, mouse_x: i32, mouse_y: i32, camera: Option<&Camera>) -> bool {
//         let mut ret = false;
//         let mut center = self.position.clone();
//         let mut points = self.points.borrow_mut().clone();
//         if let Some(camera) = camera {
//             camera.points_to_camera(&mut points);
//             camera.vector2d_to_camera(&mut center);
//         }
//         let distance_squared =
//             (center.x as i32 - mouse_x).pow(2) + (center.y as i32 - mouse_y).pow(2);
//         if distance_squared < 100 {
//             return true;
//         }

//         let mut next_point = points[0];
//         for (i, point) in points.iter().enumerate() {
//             if i == points.len() - 1 {
//                 next_point = points[0];
//             } else {
//                 next_point = points[i + 1];
//             }
//             if ((point.y >= mouse_y && next_point.y < mouse_y)
//                 || (point.y < mouse_y && next_point.y >= mouse_y))
//                 && (mouse_x
//                     < (next_point.x - point.x) * (mouse_y - point.y) / (next_point.y - point.y)
//                         + point.x)
//             {
//                 ret = !ret;
//             }
//         }
//         ret
//     }

//     type Input = MenuStateHandler<'g>;
// }
