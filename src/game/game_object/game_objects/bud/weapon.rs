use std::{fs, rc::Rc};

use sdl2::{
    rect::{Point, Rect},
    render::Texture,
};

#[derive(Clone)]
pub struct Weapon {
    damage_map: Rc<Vec<(Point, u8)>>,
    range: u8,
    pub weapon_enum: WeaponEnum,
}
impl Weapon {
    pub fn default() -> Weapon {
        Weapon {
            damage_map: Rc::new(vec![(Point::new(0, 1), 3), (Point::new(0, 2), 3)]),
            weapon_enum: WeaponEnum::Sword,
            range: 0,
        }
    }
    // pub fn from_file(file: &str) -> Weapon {
    //     let mut damage_map = Vec::new();
    //     let contents = fs::read_to_string(file).expect(&format!("No Weapon Found in {}", file));
    //     for line in contents.lines() {
    //         for num_str in line.split(", ") {
    //             let num_str = num_str.to_string();
    //             let num = num_str.parse();
    //         }
    //     }
    // Weapon {
    //     damage_map: Rc::new(damage_map),
    // }
    // }
    pub fn from_enum(weapon_enum: WeaponEnum) -> Weapon {
        Weapon {
            damage_map: Rc::new(match weapon_enum {
                WeaponEnum::Sword => vec![(Point::new(0, 1), 3), (Point::new(0, 2), 3)],
                WeaponEnum::Axe => vec![
                    (Point::new(0, 1), 2),
                    (Point::new(1, 1), 2),
                    (Point::new(-1, 1), 2),
                ],
                WeaponEnum::Dagger => vec![(Point::new(0, 1), 3)],
                WeaponEnum::Shield => vec![(Point::new(0, 1), 3)],
                WeaponEnum::Bow => vec![(Point::new(0, 1), 1)],
                WeaponEnum::Crossbow => vec![(Point::new(0, 1), 2)],
                WeaponEnum::Slingshot => vec![
                    (Point::new(0, 1), 1),
                    (Point::new(1, 2), 1),
                    (Point::new(1, 1), 1),
                    (Point::new(-1, 1), 1),
                ],
                WeaponEnum::Javelin => vec![(Point::new(0, 1), 3)],
            }),
            range: match weapon_enum {
                WeaponEnum::Bow => 6,
                WeaponEnum::Crossbow => 4,
                WeaponEnum::Slingshot => 4,
                WeaponEnum::Javelin => 3,
                _ => 0,
            },
            weapon_enum,
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeaponEnum {
    Sword,
    Axe,
    Dagger,
    Shield,
    Bow,
    Crossbow,
    Slingshot,
    Javelin,
}

impl WeaponEnum {
    pub fn get_weapon(weapon_index: u8) -> (Weapon, String, Rect) {
        match weapon_index {
            0 => (
                Weapon::from_enum(WeaponEnum::Sword),
                String::from("sword"),
                Rect::new(20, 60, 5, 15),
            ),
            1 => (
                Weapon::from_enum(WeaponEnum::Axe),
                String::from("axe"),
                Rect::new(20, 60, 5, 15),
            ),
            2 => (
                Weapon::from_enum(WeaponEnum::Dagger),
                String::from("dagger"),
                Rect::new(20, 60, 5, 15),
            ),
            3 => (
                Weapon::from_enum(WeaponEnum::Shield),
                String::from("shield"),
                Rect::new(20, 60, 5, 15),
            ),
            4 => (
                Weapon::from_enum(WeaponEnum::Bow),
                String::from("bow"),
                Rect::new(20, 60, 10, 5),
            ),
            5 => (
                Weapon::from_enum(WeaponEnum::Crossbow),
                String::from("crossbow"),
                Rect::new(20, 60, 5, 15),
            ),
            6 => (
                Weapon::from_enum(WeaponEnum::Slingshot),
                String::from("slingshot"),
                Rect::new(20, 60, 5, 15),
            ),
            7 => (
                Weapon::from_enum(WeaponEnum::Javelin),
                String::from("javelin"),
                Rect::new(20, 60, 5, 15),
            ),
            _ => unreachable!(),
        }
    }
    pub fn get_index(&self) -> u8 {
        match self {
            WeaponEnum::Sword => 0,
            WeaponEnum::Axe => 1,
            WeaponEnum::Dagger => 2,
            WeaponEnum::Shield => 3,
            WeaponEnum::Bow => 4,
            WeaponEnum::Crossbow => 5,
            WeaponEnum::Slingshot => 6,
            WeaponEnum::Javelin => 7,
        }
    }
}

#[derive(Clone)]
pub struct WeaponInfo<'t> {
    pub weapon: Weapon,
    pub weapon_texture: Rc<Texture<'t>>,
    pub weapon_rect: Rect,
}

impl<'t> WeaponInfo<'t> {
    pub fn new(weapon: Weapon, weapon_texture: Rc<Texture<'t>>, weapon_rect: Rect) -> WeaponInfo {
        WeaponInfo {
            weapon,
            weapon_texture: weapon_texture,
            weapon_rect,
        }
    }
    pub fn default(texture: Rc<Texture<'t>>) -> WeaponInfo<'t> {
        let (weapon, _, weapon_rect) = WeaponEnum::get_weapon(0);
        WeaponInfo {
            weapon,
            weapon_texture: texture,
            weapon_rect,
        }
    }
}
