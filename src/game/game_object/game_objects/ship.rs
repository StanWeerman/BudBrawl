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
        game_info::GameInfo,
        game_object::{
            game_objects::{
                shell::{InitialShellData, Shell},
                GameObjectEnum,
            },
            GameObject, SuperGameObject,
        },
        game_state::StateInfo,
        input::Input,
        menu::menu_state::menu_states::{MenuStateEnum, MenuStateHandler},
    },
    vector2d::Vector2d,
};

pub struct Ship<'s> {
    position: Vector2d,
    ship_data: Rc<RefCell<ShipData<'s>>>,
    points: Rc<RefCell<Vec<Point>>>,
    hovered: bool,
    pressed: bool,
    do_radar: bool,
    active_radar: bool,
    do_shoot: bool,
    radar_points: Vec<Point>,
}

impl<'s> Ship<'s> {
    pub fn new(position: Vector2d, initial_ship_data: Rc<InitialShipData<'s>>) -> Self {
        let points = Rc::new(RefCell::new(initial_ship_data.points.clone()));
        Self {
            position,
            points,
            ship_data: Rc::new(RefCell::new(ShipData::default(initial_ship_data))),
            hovered: false,
            pressed: false,
            do_radar: true,
            active_radar: false,
            do_shoot: true,
            radar_points: vec![],
        }
    }

    // pub fn check_select(&mut self, e: &EventPump, camera: &Camera) -> bool {
    //     let m = e.mouse_state();
    //     let bounds = self.in_bounds(m.x(), m.y());
    //     if m.left() && bounds {
    //         return true;
    //     }
    //     return false;
    // }

    // pub fn in_bounds(&self, mouse_x: i32, mouse_y: i32) -> bool {
    //     let mut ret = false;
    //     let mut next_point = self.points[0];
    //     for (i, point) in self.points.iter().enumerate() {
    //         if i == self.points.len() - 1 {
    //             next_point = self.points[0];
    //         } else {
    //             next_point = self.points[i + 1];
    //         }
    //         // if (((vc.y >= py && vn.y < py) || (vc.y < py && vn.y >= py))
    //         //     && (px < (vn.x - vc.x) * (py - vc.y) / (vn.y - vc.y) + vc.x))
    //         // {
    //         //     collision = !collision;
    //         // }
    //         if ((point.y >= mouse_y && next_point.y < mouse_y)
    //             || (point.y < mouse_y && next_point.y >= mouse_y))
    //             && (mouse_x
    //                 < (next_point.x - point.x) * (mouse_y - point.y) / (next_point.y - point.y)
    //                     + point.x)
    //         {
    //             ret = !ret;
    //         }
    //     }
    //     ret
    //     // (mouse_x >= self.rect.left() && mouse_x <= self.rect.right())
    //     //     && (mouse_y >= self.rect.top() && mouse_y <= self.rect.bottom())
    // }

    pub fn move_ship(&mut self, delta_time: f32) -> Point {
        let moving = self.ship_data.borrow_mut().moving;
        if moving > 0.0 {
            {
                let mut ship_data = self.ship_data.borrow_mut();
                // println!("Moving: {}", self.moving);
                ship_data.moving -= delta_time;
                ship_data.angle += (delta_time / 1000.0) * ship_data.steering_angle;
                if ship_data.speed < ship_data.new_speed {
                    ship_data.speed += (delta_time / 1000.0);
                } else if ship_data.speed > ship_data.new_speed {
                    ship_data.speed -= (delta_time / 1000.0);
                }
                // println!("{}", self.speed);
                // self.speed += (delta_time / 1000.0) * self.new_speed;
                ship_data.speed = f32::max(ship_data.speed, 0.0);
                self.position.x +=
                    ship_data.angle.to_radians().cos() * (delta_time / 100.0) * ship_data.speed;
                self.position.y +=
                    ship_data.angle.to_radians().sin() * (delta_time / 100.0) * ship_data.speed;
            }
            self.rotate_points();
        }
        return Point::new(
            (self.position.x + self.ship_data.borrow().initial.size.y / 2.0) as i32,
            (self.position.y + self.ship_data.borrow().initial.size.y / 2.0) as i32,
        );
    }

    pub fn rotate_points(&mut self) {
        let angle = self.ship_data.borrow().angle;
        for (i, point) in self
            .ship_data
            .borrow_mut()
            .initial
            .points
            .iter()
            .enumerate()
        {
            self.points.borrow_mut()[i] = self.rotate_point(
                &(*point + Point::new(self.position.x as i32, self.position.y as i32)),
                angle,
            );
        }
    }

    // fn rotate_point(&center: &Point, angle: f32, point: &pointpoint: &mut Point) {
    //     let xc = center.x;
    //     let yc = center.y;

    //     point.x = (point.x as f32 - xc) * angle.to_radians().cos()
    //         - (point.y as f32 - yc) * angle.to_radians().sin()
    //         + xc;
    //     point.y = (point.x as f32 - xc) * angle.to_radians().sin()
    //         + (point.y as f32 - yc) * angle.to_radians().cos()
    //         + yc;
    // }

    fn rotate_point(&self, point: &Point, angle: f32) -> Point {
        // let x1 = (self.position.x - point.x as f32) * self.angle.to_radians().cos()
        //     - (self.position.y - point.y as f32) * self.angle.to_radians().sin()
        //     + point.x as f32;
        // let y1 = (self.position.x - point.x as f32) * self.angle.to_radians().sin()
        //     - (self.position.y - point.y as f32) * self.angle.to_radians().cos()
        //     + point.y as f32;
        let xc = self.position.x;
        let yc = self.position.y;

        // let angle_radians = self.ship_data.borrow().angle.to_radians();
        let angle_radians = angle.to_radians();
        let x1 = (point.x as f32 - xc) * angle_radians.cos()
            - (point.y as f32 - yc) * angle_radians.sin()
            + xc;
        let y1 = (point.x as f32 - xc) * angle_radians.sin()
            + (point.y as f32 - yc) * angle_radians.cos()
            + yc;
        return Point::new(x1 as i32, y1 as i32);
    }
    fn draw_radar(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        //canvas.fill_rect(new_rect);
        //canvas.filled_circle(768 / 2, 768 / 2, 30, Color::RGB(0, 0, 0));

        let mut camera_pos = self.position.clone();
        camera.vector2d_to_camera(&mut camera_pos);
        camera_pos = Vector2d::new(camera_pos.x, camera_pos.y);
        // canvas.filled_circle(
        //     camera_pos.x as i16,
        //     camera_pos.y as i16,
        //     6 * 100,
        //     Color::RGBA(0, 15, 0, 50),
        // );
        canvas.set_scale(100.0 / camera.scale as f32, 100.0 / camera.scale as f32);
        for i in 0..4 {
            let for_x = (camera_pos.x / (100.0 / camera.scale as f32)) as i16;
            let for_y = (camera_pos.y / (100.0 / camera.scale as f32)) as i16;
            canvas.filled_circle(
                for_x,
                for_y,
                (4 - i) * ((camera.window_scale()) as i16 * 25) + 2,
                Color::RGBA(0, 255, 0, 50),
            );
            canvas.filled_circle(
                for_x,
                for_y,
                (4 - i) * ((camera.window_scale()) as i16 * 25),
                Color::RGBA(0, 15, 0, 50),
            );
        }
        canvas.set_scale(1.0, 1.0);
        // canvas.set_draw_color(Color::RGBA(0, 0, 100, 255));
        // for i in 0..(camera.blocks().1 as usize) {
        //     canvas.draw_line(
        //         Point::new(0, ((i + 1) as f32 * camera.window_scale()) as i32),
        //         Point::new(
        //             camera.window_size.0 as i32,
        //             ((i + 1) as f32 * camera.window_scale()) as i32,
        //         ),
        //     );
        // }
        canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
        for mut p in self.radar_points.clone() {
            println!("BEFORE {}, {}, scale: {}", p.x, p.y, camera.scale);
            // p.x = (((p.x as f32 / camera.window_scale()) as i32) as f32 * camera.window_scale())
            //     as i32;
            // p.x = (((p.x as f32 / camera.window_scale()) as i32) as f32 * camera.window_scale())
            //     as i32;
            // camera.point_to_camera(&mut p);
            p.x = (p.x / camera.scale) * camera.scale;
            p.y = (p.y / camera.scale) * camera.scale;
            camera.point_to_camera(&mut p);
            p.x = (((p.x as f32 / camera.window_scale()) as i32) as f32 * camera.window_scale())
                as i32;
            p.y = (((p.y as f32 / camera.window_scale()) as i32) as f32 * camera.window_scale())
                as i32;
            println!("AFTER {}, {}, scale: {}", p.x, p.y, camera.window_scale());
            canvas.fill_rect(Rect::new(
                p.x,
                p.y,
                camera.window_scale() as u32,
                camera.window_scale() as u32,
            ));
        }
    }
    fn draw_expected_points(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        let mut ship_data = self.ship_data.borrow_mut();
        let mut speed = ship_data.speed;
        let mut new_speed = ship_data.new_speed;
        let mut angle = ship_data.angle;
        let mut steering_angle = ship_data.steering_angle;
        let delta_time = 0.250 * 1000.0;
        let mut position = Vector2d::new(self.position.x, self.position.y);
        let mut points = Vec::new();

        for i in 0..32 {
            angle += (delta_time / 1000.0) * steering_angle;
            if speed < new_speed {
                speed += (delta_time / 1000.0);
            } else if speed > new_speed {
                speed -= (delta_time / 1000.0);
            }
            // println!("{}", self.speed);
            // self.speed += (delta_time / 1000.0) * self.new_speed;
            speed = f32::max(speed, 0.0);
            position.x += angle.to_radians().cos() * (delta_time / 100.0) * speed;
            position.y += angle.to_radians().sin() * (delta_time / 100.0) * speed;
            let mut point = Point::new(position.x as i32, position.y as i32);
            camera.point_to_camera(&mut point);
            points.push(point);
        }
        canvas.draw_lines(&points[..]);
    }
}

impl<'g> GameObject<'g> for Ship<'g> {
    fn get_position(&self) -> Vector2d {
        self.position.clone()
    }

    fn get_draw_values(&self) -> (Vector2d, Vector2d) {
        (
            self.position.clone(),
            self.ship_data.borrow().initial.size.clone(),
        )
    }

    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        // let (position, size) = self.get_draw_values();
        canvas.set_draw_color(Color::RGBA(139, 210, 241, 255));
        if self.ship_data.borrow().selected.get() && self.active_radar {
            self.draw_radar(canvas, camera);
        }
        let mut some_rect = Rect::from_center(
            Point::new(self.position.x as i32, self.position.y as i32),
            self.ship_data.borrow().initial.size.x as u32,
            self.ship_data.borrow().initial.size.y as u32,
        );
        camera.rect_to_camera(&mut some_rect);

        let mut shadow_rect = Rect::new(
            (self.position.x
                + (self.ship_data.borrow().angle + self.ship_data.borrow().steering_angle)
                    .to_radians()
                    .cos()
                    * 10.0
                    * self.ship_data.borrow().new_speed) as i32,
            (self.position.y
                + (self.ship_data.borrow().angle + self.ship_data.borrow().steering_angle)
                    .to_radians()
                    .sin()
                    * 10.0
                    * self.ship_data.borrow().new_speed) as i32,
            self.ship_data.borrow().initial.size.y as u32,
            self.ship_data.borrow().initial.size.y as u32,
        );
        camera.rect_to_camera(&mut shadow_rect);
        canvas.copy_ex(
            &self.ship_data.borrow().initial.texture,
            None,
            some_rect,
            self.ship_data.borrow().angle as f64,
            None,
            false,
            false,
        );
        if self.ship_data.borrow().selected.get() {
            if self.ship_data.borrow().moving <= 0.0 {
                self.draw_expected_points(canvas, camera);
            }
        }
        let mut points = self.points.borrow_mut().clone();
        camera.points_to_camera(&mut points);
        points.push(points[0]);
        canvas.draw_lines(&points[..]);

        let mut center = self.position.clone();
        camera.vector2d_to_camera(&mut center);
        canvas.circle(
            center.x as i16,
            center.y as i16,
            5,
            Color::RGB(255, 255, 255),
        );

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
        if gi.input.is_pressed(Keycode::R) && self.do_radar {
            self.do_radar = false;
            println!("?: {}", self.do_radar);
            self.active_radar = !self.active_radar;
        } else if gi.input.is_released(Keycode::R) {
            self.do_radar = true;
        }
        if (self.active_radar) {
            self.radar_points = collisions.get_squares(gi.camera.scale);
        }
        self.move_ship(_delta_time);

        if self.ship_data.borrow().selected.get()
            && gi.input.is_pressed(Keycode::M)
            && self.do_shoot
        {
            self.do_shoot = false;
            si.add_object(Rc::new(RefCell::new(Shell::new(
                self.position.clone(),
                Rc::clone(&self.ship_data.borrow().initial.shell_data),
                // Rc::new(self.ship_data.borrow().selected.get()),
                Rc::clone(&self.ship_data.borrow().selected),
            ))));
        } else if gi.input.is_released(Keycode::M) {
            self.do_shoot = true;
        }

        true
    }
}

impl<'g> Colliding for Ship<'g> {
    fn has_collided(&mut self, other: &dyn Colliding) {
        todo!()
    }

    fn get_collider(&self) -> Rc<RefCell<Vec<Point>>> {
        Rc::clone(&self.points)
    }
}

pub struct InitialShipData<'s> {
    points: Vec<Point>,
    max_speed: f32,
    max_angle: f32,
    texture: Rc<Texture<'s>>,
    size: Vector2d,
    shell_data: Rc<InitialShellData<'s>>,
}

pub struct ShipData<'s> {
    initial: Rc<InitialShipData<'s>>,
    angle: f32,
    steering_angle: f32,
    speed: f32,
    new_speed: f32,
    moving: f32,
    selected: Rc<Cell<bool>>,
}

impl<'s> ShipData<'s> {
    pub fn default(initial: Rc<InitialShipData>) -> ShipData {
        ShipData {
            initial,
            angle: 0.0,
            steering_angle: 0.0,
            speed: 0.0,
            new_speed: 0.0,
            moving: 0.0,
            selected: Rc::new(Cell::new(false)),
        }
    }
    pub fn go(&mut self) {
        self.moving = 8000.0;
    }

    pub fn turn(&mut self, angle: f32) {
        // println!(
        //     "{} : {}",
        //     self.steering_angle.abs() + angle,
        //     self.steering_angle
        // );
        if (self.steering_angle + angle).abs() < 50.0 {
            self.steering_angle += angle;
        }
    }
    pub fn gas(&mut self, speed: f32) {
        // println!(
        //     "{} : {},{}",
        //     self.new_speed.abs() + speed,
        //     self.new_speed,
        //     self.speed
        // );
        if (self.new_speed + speed).abs() < 20.0 && (self.new_speed + speed).abs() > 0.0 {
            self.new_speed += speed;
        }
    }
    pub fn select(&mut self) {
        self.selected.set(true);
    }
    pub fn unselect(&mut self) {
        self.selected.set(false);
    }
}

const FLETCHER_HEIGHT: i32 = 114;
const FLETCHER_WIDTH: i32 = 12;

impl<'s> InitialShipData<'s> {
    pub fn default(texture: Rc<Texture<'s>>) -> InitialShipData {
        InitialShipData {
            points: vec![
                Point::new(-FLETCHER_HEIGHT / 2, -FLETCHER_WIDTH / 2 / 2),
                Point::new(-FLETCHER_HEIGHT / 2 / 2, -FLETCHER_WIDTH / 2),
                Point::new(FLETCHER_HEIGHT / 2 / 2, -FLETCHER_WIDTH / 2),
                Point::new(FLETCHER_HEIGHT / 2, -FLETCHER_WIDTH / 2 / 2),
                Point::new(FLETCHER_HEIGHT / 2, FLETCHER_WIDTH / 2 / 2),
                Point::new(FLETCHER_HEIGHT / 2 / 2, FLETCHER_WIDTH / 2),
                Point::new(-FLETCHER_HEIGHT / 2 / 2, FLETCHER_WIDTH / 2),
                Point::new(-FLETCHER_HEIGHT / 2, FLETCHER_WIDTH / 2 / 2),
            ],
            max_speed: 0.0,
            max_angle: 0.0,
            shell_data: Rc::new(InitialShellData::default(Rc::clone(&texture))),
            texture: texture,
            size: Vector2d::new(FLETCHER_HEIGHT as f32, FLETCHER_WIDTH as f32),
        }
    }
}

// impl<'g> SuperGameObject<'g> for Ship<'g> {}

impl<'g> Button<'g> for Ship<'g> {
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

    fn action(&mut self, input: &mut Self::Input) {
        input.load_menu(MenuStateEnum::Ship(Some(Rc::clone(&self.ship_data))));
    }

    fn in_bounds(&self, mouse_x: i32, mouse_y: i32, camera: Option<&Camera>) -> bool {
        let mut ret = false;
        let mut center = self.position.clone();
        let mut points = self.points.borrow_mut().clone();
        if let Some(camera) = camera {
            camera.points_to_camera(&mut points);
            camera.vector2d_to_camera(&mut center);
        }
        let distance_squared =
            (center.x as i32 - mouse_x).pow(2) + (center.y as i32 - mouse_y).pow(2);
        if distance_squared < 100 {
            return true;
        }

        let mut next_point = points[0];
        for (i, point) in points.iter().enumerate() {
            if i == points.len() - 1 {
                next_point = points[0];
            } else {
                next_point = points[i + 1];
            }
            if ((point.y >= mouse_y && next_point.y < mouse_y)
                || (point.y < mouse_y && next_point.y >= mouse_y))
                && (mouse_x
                    < (next_point.x - point.x) * (mouse_y - point.y) / (next_point.y - point.y)
                        + point.x)
            {
                ret = !ret;
            }
        }
        ret
    }

    type Input = MenuStateHandler<'g>;
}
