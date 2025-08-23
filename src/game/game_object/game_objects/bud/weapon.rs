use std::{fs, rc::Rc};

use sdl2::rect::Point;

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
