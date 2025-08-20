// For Soundeffects: https://www.leshylabs.com/apps/sfMaker/
#![allow(warnings)]
extern crate sdl2;

use sdl2::{
    mixer::{open_audio, AUDIO_S16LSB, DEFAULT_CHANNELS},
    render::BlendMode,
};

pub mod game;
use crate::game::Game;

pub mod vector2d;

// pub mod menu;
// use crate::menu::Menu;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let window = sdl_context
        .video()
        .unwrap()
        .window("Bud Brawl", 1000, 1000)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    // _ = canvas.set_scale(3.0, 3.0);

    let _audio = sdl_context.audio()?;
    let _mixer = sdl2::mixer::init(sdl2::mixer::InitFlag::all());
    let frequency = 44100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_a_size = 1024;
    open_audio(frequency, format, channels, chunk_a_size).expect("...");

    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
    canvas.set_blend_mode(BlendMode::Blend);

    let texture_creator = canvas.texture_creator();
    let mut game = Game::new(canvas, sdl_context.event_pump()?, &texture_creator);
    // game.make_textures(&texture_creator);
    // game.make_music();
    game.run()
}
