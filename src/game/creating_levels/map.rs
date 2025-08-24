use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

use sdl2::render::Texture;

use crate::game::game_object::game_objects::tiles::tile_object::TileObject;
use crate::vector2d::Vector2d;

#[derive(Debug)]
pub struct Map {
    pub rules: HashMap<u32, RuleSet>,
    pub tiles: (Vec<u32>, (u32, u32)),
}

impl<'t> Map {
    pub fn new(rule_files: Vec<&str>, tile_map_file: &str) -> Map {
        Map {
            rules: Map::generate_rules(rule_files),
            tiles: Map::generate_tiles(tile_map_file),
        }
    }
    pub fn make_tile_objects(&self, tex: Rc<RefCell<Texture<'t>>>) -> Vec<TileObject<'t>> {
        let mut ret = Vec::new();
        let (ref tiles, (width, height)) = self.tiles;
        for (position_index, tile) in tiles.iter().enumerate() {
            if *tile != 0 {
                let rule_set = self.rules.get(tile).unwrap();
                for (index, rule) in rule_set.rules.iter().enumerate() {
                    if rule.check_rule(
                        [
                            *(tiles.get((position_index as isize - width as isize - 1) as usize))
                                .unwrap_or(&u32::default()),
                            *(tiles.get((position_index as isize - width as isize) as usize))
                                .unwrap_or(&u32::default()),
                            *(tiles.get((position_index as isize - width as isize + 1) as usize))
                                .unwrap_or(&u32::default()),
                            *(tiles.get((position_index as isize - 1) as usize))
                                .unwrap_or(&u32::default()),
                            *(tiles.get(position_index + 1)).unwrap_or(&u32::default()),
                            *(tiles.get(position_index + width as usize - 1))
                                .unwrap_or(&u32::default()),
                            *(tiles.get(position_index + width as usize))
                                .unwrap_or(&u32::default()),
                            *(tiles.get(position_index + width as usize + 1))
                                .unwrap_or(&u32::default()),
                        ],
                        &rule_set.types,
                    ) {
                        // println!("Hmm");
                        ret.push(TileObject::new(
                            Vector2d::new(
                                (position_index as u32 % width) as f32,
                                (position_index as u32 / width) as f32,
                            ),
                            Vector2d::new(16.0, 16.0),
                            Rc::clone(&tex),
                            true,
                            Vector2d::new(16.0 * (index % 8) as f32, 16.0 * (index / 8) as f32),
                        ));
                        break;
                    }
                }
            }
        }
        ret
    }
    pub fn generate_tiles(tile_map_file: &str) -> (Vec<u32>, (u32, u32)) {
        let file_path = format!("assets/tile_map/tile_maps/{tile_map_file}");
        let contents: Vec<String> = fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(String::from)
            .map(|s| s.split_whitespace().collect())
            .collect();
        let mut vec = Vec::new();
        let x = contents[0].len() as u32;
        let y = contents.len() as u32;
        for line in contents {
            for c in line.chars() {
                vec.push(c.to_digit(10).unwrap());
            }
        }
        // println!("{:?}, ({}, {})", contents, x, y);
        (vec, (x, y))
    }
    pub fn generate_rules(rule_files: Vec<&str>) -> HashMap<u32, RuleSet> {
        let mut rules = HashMap::new();
        for rule_file in rule_files {
            Map::generate_rule_set(rule_file, &mut rules);
        }
        rules
    }
    pub fn generate_rule_set(rule_file: &str, rules: &mut HashMap<u32, RuleSet>) {
        let mut number = 0;
        let mut rule_set = RuleSet::default();
        // let mut types = Vec::new();

        let file_path = format!("assets/tile_map/rule_sets/{rule_file}");
        let contents: Vec<String> = fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(String::from)
            .map(|s| s.split_whitespace().collect())
            .collect();
        let mut count = 0;
        for line in contents {
            if count == 0 {
                for c in line.chars().take(1) {
                    number = c.to_digit(10).unwrap();
                }
                count += 1;
                // break;
            } else if count == 1 {
                for (index, c) in line.chars().enumerate() {
                    if let Some(num) = c.to_digit(10) {
                        // println!("{:?}", num);
                        // types.push(num)
                        rule_set.add_type(num);
                    }
                }
                count += 1;
                // break;
            } else {
                let mut surround = [0; 8];

                // println!("Line: {line}");
                for (index, c) in line.chars().take(8).enumerate() {
                    if let Some(num) = c.to_digit(10) {
                        // println!("{:?}", num);
                        surround[index] = num;
                    }
                }
                rule_set.insert(Rule::new(surround), count - 2);
                count += 1;
            }
        }
        rules.insert(number, rule_set);
    }
}

// type RuleSet = BTreeMap<Rule, u32>;
//
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct RuleSet {
    rules: Vec<Rule>,
    types: Vec<u32>,
    // tile_map: u32,
}
impl RuleSet {
    pub fn new(types: Vec<u32>, tile_map: u32) -> Self {
        Self {
            rules: Vec::new(),
            types,
            // tile_map,
        }
    }
    pub fn default() -> Self {
        Self {
            rules: Vec::new(),
            types: Vec::new(),
            // tile_map,
        }
    }
    pub fn set_types(&mut self, types: Vec<u32>) {
        self.types = types;
    }
    pub fn add_type(&mut self, num: u32) {
        self.types.push(num);
    }
    pub fn insert(&mut self, rule: Rule, tile: u32) {
        self.rules.push(rule);
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Rule {
    surround: [u32; 8],
    // types: &'t Vec<u32>,
}

impl Rule {
    pub fn new(surround: [u32; 8]) -> Rule {
        Rule { surround }
    }
    pub fn check_rule(&self, other_surround: [u32; 8], types: &Vec<u32>) -> bool {
        // println!("{:?}, {:?}", other_surround, self.surround);
        let mut index = 0;
        for i in other_surround {
            if self.surround[index] == 9 {
            } else if (self.surround[index] == 0 && i != 0) || (self.surround[index] == 1 && i == 0)
            {
                return false;
            } else if !types.contains(&i) && i != 0 {
                return false;
            }
            index += 1;
        }
        return true;
    }
}
