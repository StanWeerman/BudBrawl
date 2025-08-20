use std::collections::HashMap;

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseState, EventPump};

#[derive(PartialEq)]
pub enum KeyState {
    None,
    Pressed,
    Released,
}

pub struct Input {
    pub key_map: HashMap<Keycode, KeyState>,
    pub mouse_state: MouseState,
}

fn get_keys() -> HashMap<Keycode, KeyState> {
    let mut key_map = HashMap::new();
    add_keys(
        &mut key_map,
        &[
            Keycode::A,
            Keycode::D,
            Keycode::W,
            Keycode::S,
            Keycode::Down,
            Keycode::Up,
            Keycode::Right,
            Keycode::Left,
            Keycode::M,
            Keycode::R,
            Keycode::T,
            Keycode::Num1,
            Keycode::Num2,
            Keycode::Space,
            Keycode::Return,
        ],
    );
    return key_map;
}

fn add_keys(key_map: &mut HashMap<Keycode, KeyState>, keys: &[Keycode]) {
    for key in keys {
        key_map.insert(*key, KeyState::None);
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            key_map: get_keys(),
            mouse_state: MouseState::from_sdl_state(0),
        }
    }
    // pub fn get_state<'k>(&self, key_code: Keycode) -> Option<&'k KeyState> {
    //     self.key_map.get(&key_code)
    // }
    pub fn is_pressed(&self, key_code: Keycode) -> bool {
        // match self.key_map.get(&key_code) {
        match self.key_map.get(&key_code) {
            Some(KeyState::Pressed) => true,
            None | Some(_) => false,
        }
    }
    pub fn is_released(&self, key_code: Keycode) -> bool {
        // match self.key_map.get(&key_code) {
        match self.key_map.get(&key_code) {
            Some(KeyState::Released) => true,
            None | Some(_) => false,
        }
    }
    pub fn get_input(&mut self, event_pump: &mut EventPump, running: &mut bool) {
        self.mouse_state = event_pump.mouse_state();
        // for (_, key_state) in &self.key_map {
        //     match key_state {
        //         KeyState::None => println!("None"),
        //         KeyState::Pressed => println!("Pressed"),
        //         KeyState::Released => println!("Released"),
        //     }
        // }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *running = false;
                }
                Event::KeyDown {
                    keycode: Some(some_key),
                    ..
                // } => match self.key_map.get_mut(&some_key) {
                } => match self.key_map.get_mut(&some_key) {
                    Some(key) => *key = KeyState::Pressed,
                    None => {}
                },
                Event::KeyUp {
                    keycode: Some(some_key),
                    ..
                // } => match self.key_map.get_mut(&some_key) {
                } => match self.key_map.get_mut(&some_key) {
                    Some(key) => *key = KeyState::Released,
                    None => {}
                },
                _ => {}
            }
        }
    }
}
