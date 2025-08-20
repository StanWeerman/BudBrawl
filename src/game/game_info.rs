use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    rc::Rc,
};

use sdl2::{
    image::LoadTexture,
    mixer::{Chunk, Music},
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::game::game_state::{
    game_states::{home_state::HomeState, GameStateHandler},
    GameState,
};

use super::{camera::Camera, input::Input};

pub struct GameInfo<'g> {
    pub running: bool,
    pub camera: Camera,
    pub input: Input,
    pub texture_creator: &'g TextureCreator<WindowContext>,
    pub textures: HashMap<String, Rc<RefCell<Texture<'g>>>>,
    pub musics: HashMap<String, Rc<Music<'g>>>,
    pub sound_effects: HashMap<String, Rc<RefCell<Chunk>>>,
    pub restart: bool,
    pub game_state_handler: GameStateHandler<'g>,
}

impl<'g> GameInfo<'g> {
    pub fn new(
        canvas: &Canvas<Window>,
        texture_creator: &'g TextureCreator<WindowContext>,
    ) -> GameInfo<'g> {
        GameInfo {
            running: true,
            camera: Camera::default(),
            input: Input::new(),
            texture_creator,
            textures: make_map(
                "assets/textures",
                &|file| Rc::new(RefCell::new(texture_creator.load_texture(&file).unwrap())),
                &["png", "jpg", "jpeg"],
            ),
            musics: make_map(
                "assets/musics",
                &|file| Rc::new(Music::from_file(&file).unwrap()),
                &["wav", "mp3"],
            ),
            sound_effects: make_map(
                "assets/sound_effects",
                &|file| {
                    // println!("Making sound {}", file);
                    Rc::new(RefCell::new(Chunk::from_file(file).unwrap()))
                },
                &["wav", "mp3"],
            ),
            restart: false,
            game_state_handler: GameStateHandler::new(),
        }
    }
}

#[doc = "# Generic mapping function\nTo make a `HashMap<String, R>`\nFor any generic resource `R` from a main directory\n\n###### Pass in:\n1. `&str` Directory\n2. `&dyn Fn(&str) -> R` Closure to make new resource\n3. `&[&str]` of file extensions, after the dot\n\n###### Ex:\n\n```\nmake_map(\n\"assets/textures\",\n&|file| Rc::new(texture_creator.load_texture(&file).unwrap()),\n&[\"png\", \"jpg\", \"jpeg\"])\n```\nFor a `HashMap<String, Rc<Texture<\'g>>>`"]
pub fn make_map<R>(
    directory: &str,
    new_resource: &dyn Fn(&str) -> R,
    extensions: &[&str],
) -> HashMap<String, R> {
    let mut ret = HashMap::new();
    let mut dirs = VecDeque::new();
    dirs.push_back(String::from(directory));
    while let Some(dir) = dirs.pop_front() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            dirs.push_back(String::from(
                                entry.path().as_os_str().to_str().unwrap(),
                            ));
                        } else {
                            let tex_name_ = entry.file_name();
                            let tex_name = tex_name_.as_os_str().to_str().unwrap();
                            let dot = tex_name.find('.').unwrap();
                            if check_extension(&tex_name[dot + 1..], &extensions) {
                                ret.insert(
                                    String::from(&tex_name[..dot]),
                                    new_resource(entry.path().as_os_str().to_str().unwrap()),
                                );
                            }
                        }
                    } else {
                        println!("Couldn't get file type for {:?}", entry.path());
                    }
                }
            }
        }
    }
    ret
}

fn check_extension(extension: &str, extensions: &[&str]) -> bool {
    for ext in extensions {
        if *ext == extension {
            return true;
        }
    }
    false
}
