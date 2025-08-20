use libnoise::{Generator, Generator1D, Simplex, Source, Worley};
use std::rc::Rc;

use sdl2::{
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    pixels::{Color, PixelFormat, PixelFormatEnum},
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    sys::{SDL_AllocFormat, SDL_MapRGB},
    video::{Window, WindowContext},
    EventPump,
};

use crate::{
    game::{
        button::{Button, NoButton},
        camera::Camera,
        collision_system::collisions::Collisions,
        game_info::GameInfo,
        game_object::{game_objects::GameObjectEnum, GameObject, SuperGameObject},
        game_state::StateInfo,
        input::Input,
    },
    vector2d::Vector2d,
};

pub struct Ground<'s> {
    position: Vector2d,
    scale: i32,
    tex: Texture<'s>,
    do_zoom_in: bool,
    do_zoom_out: bool,
}

impl<'s> Ground<'s> {
    pub fn new(position: Vector2d, scale: i32, mut tex: Texture<'s>) -> Self {
        Self {
            position,
            scale: scale,
            tex,
            do_zoom_in: true,
            do_zoom_out: true,
        }
    }
    fn gen_ground(&mut self, gi: &mut GameInfo<'s>) {
        let (x_size, y_size) = gi.camera.blocks();
        let (x_size, y_size) = (x_size as usize, y_size as usize);
        self.tex = gi
            .texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, x_size as u32, y_size as u32)
            .map_err(|e| e.to_string())
            .unwrap();
        let (octaves, mut frequency, lacunarity, persistence) = (10, 0.00001, 1.6, 0.7);
        // frequency = 0.1;
        // frequency / self.scale as f64;
        // let generator = Source::simplex(2123)
        //     .fbm(octaves, frequency, lacunarity, persistence)
        //     .neg()
        //     .blend(
        //         // apply blending...
        //         //Source::simplex(4).fbm(octaves, 0.001, lacunarity, persistence),
        //         Source::constant(0.0),
        //         Source::worley(430)
        //             .fbm(3, 0.01, 1.0, 0.7)
        //             .neg()
        //             .scale([0.5, 0.5]),
        //     );
        // let gen = Source::worley(1)
        //     .fbm(5, 0.0001, 1.2, 0.7)
        //     .neg()
        //     .product(Source::worley(2).fbm(5, 0.0001, 1.2, 0.7).neg());
        //let gen = Source::constant(1.0).select(Source::constant(0.0), gen, 0.03, 1.0);

        let generator = Source::simplex(2123).fbm(octaves, frequency, lacunarity, persistence);

        let rounded_x = (self.position.x as i32 / self.scale) * self.scale;

        let rounded_y = (self.position.y as i32 / self.scale) * self.scale;

        //.select(Source::constant(0.0), gen.clone(), 0.03, 1.0);
        let mut vals = vec![0f64; x_size * y_size + 1];
        for x in 0..x_size {
            for y in 0..y_size {
                let height = 100.0
                    * generator.sample([
                        0.0 + rounded_x as f64 + self.scale as f64 * (x as f32) as f64,
                        rounded_y as f64 + self.scale as f64 * (y as f32) as f64,
                    ]);
                vals[y_size * x + y] = height;
            }
        }
        self.tex
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for x in 0..x_size {
                    for y in 0..y_size {
                        let offset = y * pitch + x * 3;

                        buffer[offset] = 0;
                        buffer[offset + 1] = 0;
                        buffer[offset + 2] = 0;

                        let height = vals[x * y_size + y];
                        if height > 52.0 {
                            //Caps
                            //let mut val = height.powf(1.2) as u8 + 100;
                            let mut val = 255;
                            if (val < 255 - 100) {
                                val += 100;
                            } else {
                                val = 255;
                            }
                            buffer[offset] = val;
                            buffer[offset + 1] = val;
                            buffer[offset + 2] = val;
                        } else if height > 45.0 {
                            //High Mountains
                            let mut val = height.powf(1.4) as u8 - 75;
                            if (val < 255 - 10) {
                                val += 10;
                            } else {
                                val = 255;
                            }
                            buffer[offset] = val;
                            buffer[offset + 1] = val;
                            buffer[offset + 2] = val;
                        } else if height > 40.0 {
                            //Mountains / Desert
                            let mut val = height.powf(1.3) as u8;
                            buffer[offset] = 30 + val;
                            buffer[offset + 1] = 40 + val;
                            buffer[offset + 2] = val;
                        } else if height > 33.0 {
                            //Grassy
                            let mut val = (height.powf(1.3) / 1.5) as u8;
                            if (val < 255 - 75) {
                                val += 75;
                            } else {
                                val = 255;
                            }
                            buffer[offset + 1] = val;
                        } else if height > 26.5 {
                            //Jungle
                            let val = (height.powf(2.0) / 10.0) as u8;
                            buffer[offset + 1] = val;
                        } else if height > 25.0 {
                            //Beaches
                            buffer[offset] = 173 + 50 - (2.0 * height) as u8;
                            //255, 214, 135
                            buffer[offset + 1] = 145 + 100 - (4.0 * height) as u8;

                            buffer[offset + 2] = 78 + 100 - (4.0 * height) as u8;
                        } else {
                            //Water
                            buffer[offset + 1] = 0;
                            buffer[offset + 2] = 100 + height.powf(1.5) as u8;
                        }
                    }
                }
            })
            .unwrap();
    }
}

impl<'g> GameObject<'g> for Ground<'g> {
    fn get_position(&self) -> Vector2d {
        self.position.clone()
    }

    fn get_draw_values(&self) -> (Vector2d, Vector2d) {
        (self.position.clone(), Vector2d::new(1.0, 1.0))
    }

    fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        // let (position, size) = self.get_draw_values();
        //canvas.set_draw_color(Color::RGBA(139, 210, 241, 255));
        let blocks = camera.blocks();
        // let hmm = Vector2d::new(
        //     self.position.x - blocks.0 as f32 / 2.0 * self.scale as f32,
        //     self.position.y - blocks.1 as f32 / 2.0 * self.scale as f32,
        // );
        // camera.set_camera(1.0, &hmm, self.scale);
        let mut some_rect = Rect::from_center(
            Point::new(
                camera.window_size.0 as i32 / 2,
                camera.window_size.1 as i32 / 2,
            ),
            blocks.0 * camera.window_scale() as u32,
            blocks.1 * camera.window_scale() as u32,
        );
        canvas.copy_ex(
            &self.tex, None, some_rect, 0.0, None,
            // Point::new(self.position.x as i32 + 10, self.position.y as i32 + 10),
            // some_rect.top_left() + Point::new(0, 0),
            false, false,
        );
    }
    fn update(
        &mut self,
        _delta_time: f32,
        collisions: &mut Collisions,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) -> bool {
        if (self.position.x != gi.camera.position.x
            || self.position.y != gi.camera.position.y
            || self.scale != gi.camera.scale)
        {
            self.position = gi.camera.position.clone();
            self.scale = gi.camera.scale;
            self.gen_ground(gi);
        }
        true
    }
}

impl<'g> NoButton for Ground<'g> {}
