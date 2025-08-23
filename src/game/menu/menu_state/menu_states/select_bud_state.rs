use std::{cell::RefCell, rc::Rc};

use sdl2::{
    gfx::primitives::DrawRenderer,
    image::LoadTexture,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::game::{
    button::{Button, HoverMenuButton, MenuButton},
    camera::Camera,
    effect_system::effects::self_effect::{
        BulwarkEffect, FighterEffect, MendingEffect, ScoutEffect, SelfEffect,
    },
    game_info::GameInfo,
    game_object::game_objects::bud::{
        bud_data::{BudData, InitialBudData},
        weapon::Weapon,
        Bud,
    },
    game_state::game_states::select_state::SelectInfo,
    menu::menu_state::{
        menu_states::{BudEnum, MenuStateEnum},
        MenuState,
    },
};

pub struct SelectBudState<'g> {
    select_info: Option<Rc<RefCell<SelectInfo<'g>>>>,
    full_buttons: Vec<MenuButton<SelectInfo<'g>>>,
    edit_buttons: Vec<MenuButton<SelectInfo<'g>>>,
    trait_buttons: Vec<HoverMenuButton<'g, SelectInfo<'g>>>,
}

impl<'g> SelectBudState<'g> {
    pub fn new(gi: &mut GameInfo<'g>) -> Self {
        let mut full_buttons = Vec::new();
        full_buttons.push(MenuButton::new(
            Rect::new(50, 80, 50, 20),
            "Confirm",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                if select_info.team == 0 {
                    select_info.team = 1;
                } else {
                    select_info.done = true;
                }
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(20 * 0, 20, 20, 60),
            "",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(0);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(20 * 1, 20, 20, 60),
            "",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(1);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(20 * 2, 20, 20, 60),
            "",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(2);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(20 * 3, 20, 20, 60),
            "",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(3);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(20 * 4, 20, 20, 60),
            "",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(4);
            }),
        ));

        let mut edit_buttons = Vec::new();

        edit_buttons.push(MenuButton::new(
            Rect::new(0, 75, 10, 5),
            "Back",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = None;
            }),
        ));
        edit_buttons.push(MenuButton::new(
            Rect::new(10, 75, 10, 5),
            "Reset",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                if let Some(current_initial_bud_data) = select_info.get_current_initial_bud_data() {
                    current_initial_bud_data.clear_effects();
                }
            }),
        ));

        let mut trait_buttons = Vec::new();

        Self {
            select_info: None,
            full_buttons,
            edit_buttons,
            trait_buttons,
        }
    }
}

impl<'g> MenuState<'g> for SelectBudState<'g> {
    fn load(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        menu_state_enum: &MenuStateEnum<'g>,
    ) {
        match menu_state_enum {
            MenuStateEnum::InitialBudDatas(select_info) => {
                self.select_info = Some(Rc::clone(select_info));
                let select_info = select_info.borrow_mut();

                self.trait_buttons.push(HoverMenuButton::new_texture_only(
                    Rect::new(40, 0, 5, 15),
                    "",
                    Rc::clone(select_info.icon_textures.get("sword").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("sword") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.change_weapon(Weapon::default());
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description =
                            String::from("Strikes 2 tiles in front of bud for +3 damage.");
                    }),
                ));

                self.trait_buttons.push(HoverMenuButton::new_texture_only(
                    Rect::new(60, 0, 10, 5),
                    "",
                    Rc::clone(select_info.icon_textures.get("bow").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("bow") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.change_weapon(Weapon::default());
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "Bow - Hits within +6 tiles in front of bud for +1 damage.",
                        );
                    }),
                ));

                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(20, 20, 10, 10),
                    "Fighter",
                    Rc::clone(select_info.icon_textures.get("fighter").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("fighter") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(FighterEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from("Increase buds damage by +1.");
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(20, 30, 10, 10),
                    "Bulwark",
                    Rc::clone(select_info.icon_textures.get("bulwark").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("bulwark") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(BulwarkEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description =
                            String::from("Increase buds health by +1. (Increases weight)");
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(20, 40, 10, 10),
                    "Scout",
                    Rc::clone(select_info.icon_textures.get("scout").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("scout") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(ScoutEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description =
                            String::from("Increase buds speed by +1. (Decreases weight)");
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(20, 50, 10, 10),
                    "Mending",
                    Rc::clone(select_info.icon_textures.get("mending").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("mending") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(MendingEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description =
                            String::from("Restore +1 health at the start of each turn.");
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(30, 20, 10, 10),
                    "Enrage",
                    Rc::clone(select_info.icon_textures.get("fighter").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("fighter") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(FighterEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "For every bud less in your band, bud gains +1 Fighter trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(30, 30, 10, 10),
                    "Lone Wolf",
                    Rc::clone(select_info.icon_textures.get("lone_wolf").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("lone_wolf") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "For every bud less in your band, bud gains +1 Bulwark trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(30, 40, 10, 10),
                    "Chicken",
                    Rc::clone(select_info.icon_textures.get("chicken").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("chicken") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "For every bud less in your band, bud gains +1 Scout trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(30, 50, 10, 10),
                    "Emergency",
                    Rc::clone(select_info.icon_textures.get("emergency").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("emergency") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "For every bud less in your band, bud gains +1 Mending trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(40, 20, 10, 10),
                    "Inspiring",
                    Rc::clone(select_info.icon_textures.get("emergency").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("emergency") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "Rally fellow buds +1 tile(s) around bud, giving them the Fighter trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(40, 30, 10, 10),
                    "Imposing",
                    Rc::clone(select_info.icon_textures.get("imposing").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("imposing") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "Rally fellow buds +1 tile(s) around bud, giving them the Bulwark trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(40, 40, 10, 10),
                    "Peloton",
                    Rc::clone(select_info.icon_textures.get("peloton").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("peloton") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "Rally fellow buds +1 tile(s) around bud, giving them the Scout trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(40, 50, 10, 10),
                    "Rejuvenating",
                    Rc::clone(select_info.icon_textures.get("rejuvenating").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("rejuvenating") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data
                                .add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "Rally fellow buds +1 tile(s) around bud, giving them the Mending trait.",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(50, 20, 10, 10),
                    "Berserker",
                    Rc::clone(select_info.icon_textures.get("berserker").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("berserker") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "When a fellow bud has Fighter +1 tile(s) around bud, steal the trait",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(50, 30, 10, 10),
                    "Glutton",
                    Rc::clone(select_info.icon_textures.get("glutton").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("glutton") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "When a fellow bud has Bulwark +1 tile(s) around bud, steal the trait",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(50, 40, 10, 10),
                    "First Place",
                    Rc::clone(select_info.icon_textures.get("first_place").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("first_place") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "When a fellow bud has Scout +1 tile(s) around bud, steal the trait",
                        );
                    }),
                ));
                self.trait_buttons.push(HoverMenuButton::new(
                    Rect::new(50, 50, 10, 10),
                    "Parasite",
                    Rc::clone(select_info.icon_textures.get("parasite").unwrap()),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        let tex = if let Some(tex) = select_info.icon_textures.get("parasite") {
                            Some(Rc::clone(tex))
                        } else {
                            None
                        };
                        if let Some(current_initial_bud_data) =
                            select_info.get_current_initial_bud_data()
                        {
                            current_initial_bud_data.add_effect(Box::new(SelfEffect::new()), tex);
                        }
                    }),
                    Box::new(|select_info: &mut SelectInfo<'g>| {
                        select_info.trait_description = String::from(
                            "When a fellow bud has Mending +1 tile(s) around bud, steal the trait",
                        );
                    }),
                ));
            }
            _ => unreachable!(),
        }
    }

    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) -> bool {
        let mut hover = false;
        if let Some(select_info) = &self.select_info {
            let mut select_info = select_info.borrow_mut();

            if let Some(current_bud) = select_info.current_bud {
                // Edit Mode

                for button in self.edit_buttons.iter_mut() {
                    button
                        .press(&gi.input.mouse_state, &mut select_info, Some(&gi.camera))
                        .0;

                    button.draw(canvas, &gi.camera);
                }
                for button in self.trait_buttons.iter_mut() {
                    button
                        .press(&gi.input.mouse_state, &mut select_info, Some(&gi.camera))
                        .0;

                    button.draw(canvas, &gi.camera);
                }

                if let Some(current_initial_bud_data) = select_info.get_current_initial_bud_data() {
                    current_initial_bud_data.draw_initial_bud_data(0, canvas, &gi.camera);
                }

                let mut rect = Rect::new(80, 20, 20, 60);
                gi.camera.ui_rect_to_camera(&mut rect);
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas.draw_rect(rect);
                let mut point = Point::new(81, 21);
                gi.camera.ui_point_to_camera(&mut point);
                canvas.string(
                    point.x as i16,
                    point.y as i16,
                    &select_info.trait_description,
                    sdl2::pixels::Color::RGB(0, 0, 0),
                );
                // if let Some(current_initial_bud_data) = select_info.get_current_initial_bud_data() {
                //     current_initial_bud_data.debug_effects();
                // }
            } else {
                // Viewing All Buds
                for button in self.full_buttons.iter_mut() {
                    button
                        .press(&gi.input.mouse_state, &mut select_info, Some(&gi.camera))
                        .0;
                    button.draw(canvas, &gi.camera);

                    let initial_bud_datas = if select_info.team == 0 {
                        &mut select_info.initial_buds_tuple.0
                    } else {
                        &mut select_info.initial_buds_tuple.1
                    };
                    for (i, initial_bud_data) in initial_bud_datas.iter_mut().enumerate() {
                        initial_bud_data.draw_initial_bud_data(i as i32, canvas, &gi.camera);
                    }
                }
            }
        }

        hover
    }

    fn quit(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        // if self.bud_data_left.is_some() {
        //     self.bud_data_left.as_ref().unwrap().borrow_mut().unselect();
        // }
        // if self.bud_data_right.is_some() {
        //     self.bud_data_right
        //         .as_ref()
        //         .unwrap()
        //         .borrow_mut()
        //         .unselect();
        // }
    }
}
