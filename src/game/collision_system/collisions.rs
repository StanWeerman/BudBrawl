extern crate sdl2;
use std::cell::RefCell;
use std::f64::consts::PI;
use std::i32;
use std::ops::Not;
use std::rc::Rc;

use sdl2::rect::Point;
use sdl2::sys::SDL_atan2;

use crate::vector2d::Vector2d;

// pub enum ColliderType {
//     enum Solid{},
//     Semi,
// }

pub trait Colliding {
    fn has_collided(&mut self, other: &dyn Colliding);
    fn get_collider(&self) -> Rc<RefCell<Vec<Point>>>;
    //fn get_velocity(&self) -> Vector2d;
    // fn get_move_vector(&self) -> Option<Rc<RefCell<Vector2d>>> {
    //     None
    // }
}
// impl dyn Colliding {
//     fn get_collider(&mut self) -> &Collider {
//         &Collider::new(5)
//     }
// }

#[derive(Clone, PartialEq, Debug)]
pub enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Right => Side::Left,
            Side::Left => Side::Right,
        }
    }
}

pub struct Collisions<'r> {
    colliders: Vec<Rc<RefCell<dyn Colliding + 'r>>>,
}

impl<'r> Collisions<'r> {
    pub fn new(colliders: Vec<Rc<RefCell<dyn Colliding + 'r>>>) -> Self {
        Collisions { colliders }
    }
    pub fn add(&mut self, col: Rc<RefCell<dyn Colliding + 'r>>) {
        self.colliders.push(col);
    }
    pub fn test(&self) {
        println!("Helllo");
    }

    /// Remove_all is prone to bugs; make sure collissions and scenemanager are on the same page; maybe incorporate the two
    pub fn remove_all(&mut self, indexes: &[usize]) {
        for (i, index) in indexes.iter().enumerate() {
            self.colliders.remove(*index - i);
        }
    }
    fn contains(a: &Rc<RefCell<Vec<Point>>>, b: &Rc<RefCell<Vec<Point>>>) -> bool {
        true
    }
    pub fn get_squares(&mut self, scale: i32) -> Vec<Point> {
        let mut all = vec![];
        for other in &self.colliders {
            match other.try_borrow_mut() {
                Ok(mut val) => {
                    let other_col = val.get_collider();
                    for p in other_col.borrow().iter() {
                        all.push(p.clone())
                    }
                }
                Err(_) => continue,
            }
        }
        all
    }
    pub fn check_collision(&mut self, this: &mut dyn Colliding) -> bool {
        let mut ret = false;
        let col = this.get_collider();
        for other in &self.colliders {
            match other.try_borrow_mut() {
                Ok(mut val) => {
                    let other_col = val.get_collider();
                    if Collisions::contains(&col, &other_col) {
                        val.has_collided(this);
                        this.has_collided(&mut *val);
                    }
                    ret = true;
                }
                Err(_) => continue,
            }
        }
        ret
    }
    // pub fn check_raycast(
    //     &mut self,
    //     this: &dyn Colliding,
    //     start: &Vector2d,
    //     end: Vector2d,
    // ) -> Vector2d {
    //     let mut ret = false;
    //     let col = this.get_collider();
    //     let _col = Rc::clone(&col);

    //     let mut ret_end = end;
    //     let mut min = None;
    //     let mut min_distance_squared = i32::MAX;

    //     for other in &self.colliders {
    //         match other.try_borrow_mut() {
    //             Ok(mut val) => {
    //                 let other_col = val.get_collider();
    //                 let _other_col = other_col.borrow();

    //                 if let Some(new_end) = Self::contains_raycast(start, &ret_end, &_other_col) {
    //                     let new_distance_squared = ((new_end.x - start.x) as i32).pow(2)
    //                         + ((new_end.y - start.y) as i32).pow(2);
    //                     if new_distance_squared < min_distance_squared {
    //                         min_distance_squared = new_distance_squared;
    //                         min = Some(other);
    //                         ret_end = new_end;
    //                     }
    //                 }
    //                 // if let Some(side) = _col.borrow().contains(&_other_col) {
    //                 //     val.has_collided(this, side.clone());
    //                 //     this.has_collided(&mut *val, !side.clone());
    //                 // }
    //                 ret = true;
    //             }
    //             Err(_) => continue,
    //         }
    //     }
    //     if let Some(min) = min {
    //         min.borrow_mut().has_collided(this, Side::Top);
    //     }
    //     ret_end
    //     // if ret {
    //     //     Some(ret_end)
    //     // } else {
    //     //     None
    //     // }
    // }
    // fn contains_raycast(start: &Vector2d, end: &Vector2d, other: &Collider) -> Option<Vector2d> {
    //     let x1 = start.x;
    //     let y1 = start.y;
    //     let mut x2 = end.x;
    //     let mut y2 = end.y;
    //     let rx = other.a.borrow().x;
    //     let ry = other.a.borrow().y;
    //     let rw = other.b.x;
    //     let rh = other.b.y;

    //     let mut ret = None;
    //     // let mut min_distance_squared = None;
    //     // Lefts
    //     if let Some(new_end) = Self::lineline(x1, y1, x2, y2, rx, ry, rx, ry + rh) {
    //         x2 = new_end.x;
    //         y2 = new_end.y;
    //         ret = Some(Vector2d::new(x2, y2));
    //     }
    //     // Right
    //     if let Some(new_end) = Self::lineline(x1, y1, x2, y2, rx + rw, ry, rx + rw, ry + rh) {
    //         x2 = new_end.x;
    //         y2 = new_end.y;
    //         ret = Some(Vector2d::new(x2, y2));
    //     }
    //     // Top
    //     if let Some(new_end) = Self::lineline(x1, y1, x2, y2, rx, ry, rx + rw, ry) {
    //         x2 = new_end.x;
    //         y2 = new_end.y;
    //         ret = Some(Vector2d::new(x2, y2));
    //     }
    //     // Bottom
    //     if let Some(new_end) = Self::lineline(x1, y1, x2, y2, rx, ry + rh, rx + rw, ry + rh) {
    //         x2 = new_end.x;
    //         y2 = new_end.y;
    //         ret = Some(Vector2d::new(x2, y2));
    //     }
    //     ret
    // }
    fn lineline(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x4: f32,
        y4: f32,
    ) -> Option<Vector2d> {
        // calculate the distance to intersection point
        let uA = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
            / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));
        let uB = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
            / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

        // if uA and uB are between 0-1, lines are colliding
        if (uA >= 0.0 && uA <= 1.0 && uB >= 0.0 && uB <= 1.0) {
            // optionally, draw a circle where the lines meet
            let intersectionX = x1 + (uA * (x2 - x1));
            let intersectionY = y1 + (uA * (y2 - y1));
            // fill(255, 0, 0);
            // noStroke();
            // ellipse(intersectionX, intersectionY, 20, 20);

            return Some(Vector2d::new(intersectionX, intersectionY));
        }
        return None;
    }
    // pub fn check_all(&mut self, this: &mut dyn Colliding, id: &str) {
    //     /*let col = this.get_collider();
    //     let _col = Rc::clone(&col);*/
    //     for other in &self.colliders {
    //         match other.try_borrow_mut() {
    //             Ok(mut val) => {
    //                 let other_col = val.get_collider();
    //                 let _other_col = other_col.borrow();
    //                 if _other_col.id == id {
    //                     this.has_collided(&mut *val, side.clone());
    //                 }
    //             }
    //             Err(_) => continue,
    //         }
    //     }
    // }
}
