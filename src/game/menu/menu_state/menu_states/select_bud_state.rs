use std::{cell::RefCell, rc::Rc};

use sdl2::{gfx::primitives::DrawRenderer, rect::Rect, render::Canvas, video::Window};

use crate::game::{
    button::{Button, MenuButton},
    game_info::GameInfo,
    game_object::game_objects::bud::{Bud, BudData, InitialBudData},
    menu::menu_state::{
        menu_states::{BudEnum, MenuStateEnum},
        MenuState,
    },
};

pub struct SelectBudState<'g> {
    initial_bud_datas: Option<Rc<RefCell<Vec<InitialBudData<'g>>>>>,
    team: u8,
}

impl<'g> SelectBudState<'g> {
    pub fn new(gi: &mut GameInfo<'g>) -> Self {
        Self {
            initial_bud_datas: None,
            team: 0,
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
            MenuStateEnum::InitialBudDatas((team, initial_bud_datas)) => {
                self.team = *team;
                self.initial_bud_datas = Some(Rc::clone(initial_bud_datas));
            }
            _ => unreachable!(),
        }
    }

    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) -> bool {
        let mut hover = false;
        if let Some(initial_bud_datas) = &self.initial_bud_datas {
            for (i, initial_bud_data) in initial_bud_datas.borrow_mut().iter_mut().enumerate() {
                canvas.string(
                    200 * i as i16,
                    750,
                    &initial_bud_data.name,
                    sdl2::pixels::Color::RGB(0, 0, 0),
                );
                canvas.string(
                    200,
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
