use std::{cell::RefCell, rc::Rc};

use sdl2::{gfx::primitives::DrawRenderer, rect::Rect, render::Canvas, video::Window};

use crate::game::{
    button::{Button, HoverMenuButton, MenuButton},
    effect_system::effects::self_effect::SelfEffect,
    game_info::GameInfo,
    game_object::game_objects::bud::{Bud, BudData, InitialBudData},
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
    trait_buttons: Vec<HoverMenuButton<SelectInfo<'g>>>,
}

impl<'g> SelectBudState<'g> {
    pub fn new(gi: &mut GameInfo<'g>) -> Self {
        let mut full_buttons = Vec::new();
        full_buttons.push(MenuButton::new(
            Rect::new(0, 50, 100, 100),
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
            Rect::new(200 * 0, 850, 100, 100),
            "Edit",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(0);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(200 * 1, 850, 100, 100),
            "Edit",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(1);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(200 * 2, 850, 100, 100),
            "Edit",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(2);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(200 * 3, 850, 100, 100),
            "Edit",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(3);
            }),
        ));
        full_buttons.push(MenuButton::new(
            Rect::new(200 * 4, 850, 100, 100),
            "Edit",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = Some(4);
            }),
        ));

        let mut edit_buttons = Vec::new();

        edit_buttons.push(MenuButton::new(
            Rect::new(0, 850, 50, 50),
            "Back",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.current_bud = None;
            }),
        ));
        edit_buttons.push(MenuButton::new(
            Rect::new(50, 850, 50, 50),
            "Reset",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                if let Some(current_initial_bud_data) = select_info.get_current_initial_bud_data() {
                    current_initial_bud_data.clear_effects();
                }
            }),
        ));

        let mut trait_buttons = Vec::new();

        trait_buttons.push(HoverMenuButton::new(
            Rect::new(200, 200, 100, 100),
            "Fighter",
            Box::new(|select_info: &mut SelectInfo<'g>| {
                if let Some(current_initial_bud_data) = select_info.get_current_initial_bud_data() {
                    current_initial_bud_data.add_effect(Rc::new(RefCell::new(SelfEffect::new())));
                }
            }),
            Box::new(|select_info: &mut SelectInfo<'g>| {
                select_info.trait_description = String::from("Increase buds damage by +1.");
            }),
        ));

        Self {
            select_info: None,
            full_buttons,
            edit_buttons,
            trait_buttons,
        }
    }
    fn draw_initial_bud_data(
        initial_bud_data: &InitialBudData<'g>,
        index: i16,
        canvas: &mut Canvas<Window>,
    ) {
        canvas.string(
            200 * index,
            750,
            &initial_bud_data.name,
            sdl2::pixels::Color::RGB(0, 0, 0),
        );
        canvas.string(
            200 * index,
            800,
            &format!(
                "Team {} | Bud {}",
                initial_bud_data.team + 1,
                initial_bud_data.index + 1
            ),
            sdl2::pixels::Color::RGB(0, 0, 0),
        );
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
                let initial_buds_tuple = if select_info.team == 0 {
                    &mut select_info.initial_buds_tuple.0
                } else {
                    &mut select_info.initial_buds_tuple.1
                };
                Self::draw_initial_bud_data(&initial_buds_tuple[current_bud], 0, canvas);
                canvas.string(
                    800,
                    200,
                    &select_info.trait_description,
                    sdl2::pixels::Color::RGB(0, 0, 0),
                );
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
                        Self::draw_initial_bud_data(&initial_bud_data, i as i16, canvas);
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
