use std::{cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::game::{
    button::{Button, MenuButton},
    game_info::GameInfo,
    game_object::game_objects::bud::{Bud, BudData},
    menu::menu_state::{menu_states::MenuStateEnum, MenuState},
};

pub struct BudState<'g> {
    buttons: Vec<MenuButton<Rc<RefCell<BudData<'g>>>>>,
    bud_data_left: Option<Rc<RefCell<BudData<'g>>>>,
    bud_data_right: Option<Rc<RefCell<BudData<'g>>>>,
    buttons_right: Vec<MenuButton<Rc<RefCell<BudData<'g>>>>>,
}

impl<'g> BudState<'g> {
    pub fn new(gi: &mut GameInfo<'g>) -> Self {
        let mut buttons = Vec::new();
        let mut buttons_right = Vec::new();
        buttons.push(MenuButton::new(
            Rect::new(0, 50, 100, 100),
            "Right",
            Box::new(|bud_data: &mut Rc<RefCell<BudData<'g>>>| {
                // bud_data.borrow_mut().turn(1.0);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(0, 150, 100, 100),
            "Left",
            Box::new(|bud_data| {
                // bud_data.borrow_mut().turn(-1.0);
            }),
        ));

        buttons.push(MenuButton::new(
            Rect::new(0, 250, 100, 100),
            "Gas",
            Box::new(|bud_data| {
                // bud_data.borrow_mut().gas(1.0);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(0, 350, 100, 100),
            "Brake",
            Box::new(|bud_data| {
                // bud_data.borrow_mut().gas(-1.0);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(0, 450, 100, 100),
            "GO!",
            Box::new(|bud_data| {
                // bud_data.borrow_mut().go();
            }),
        ));
        buttons_right.push(MenuButton::new(
            Rect::new(450, 450, 100, 100),
            "GO!",
            Box::new(|bud_data| {
                // bud_data.borrow_mut().go();
            }),
        ));
        Self {
            buttons,
            bud_data_left: None,
            bud_data_right: None,
            buttons_right,
        }
    }
}

impl<'g> MenuState<'g> for BudState<'g> {
    fn load(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        menu_state_enum: &MenuStateEnum<'g>,
    ) {
        match menu_state_enum {
            MenuStateEnum::Bud(bud_data_left, bud_data_right) => {
                if let Some(bud_data) = bud_data_left {
                    if self.bud_data_left.is_some() {
                        self.bud_data_left.as_ref().unwrap().borrow_mut().unselect();
                    }
                    self.bud_data_left = Some(Rc::clone(bud_data));
                    self.bud_data_left.as_ref().unwrap().borrow_mut().select();
                }
                if let Some(bud_data) = bud_data_right {
                    if self.bud_data_right.is_some() {
                        self.bud_data_right
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .unselect();
                    }
                    self.bud_data_right = Some(Rc::clone(bud_data));
                    self.bud_data_right.as_ref().unwrap().borrow_mut().select();
                }
            }
            _ => unreachable!(),
        }
    }

    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) -> bool {
        let mut hover = false;
        if let Some(bud_data) = &mut self.bud_data_left {
            for button in self.buttons.iter_mut() {
                if button
                    .press(&gi.input.mouse_state, bud_data, Some(&gi.camera))
                    .0
                {
                    hover = true;
                }
                button.draw(canvas, &gi.camera);
            }
        }
        if let Some(bud_data) = &mut self.bud_data_right {
            for button in self.buttons_right.iter_mut() {
                if button
                    .press(&gi.input.mouse_state, bud_data, Some(&gi.camera))
                    .0
                {
                    hover = true;
                }
                button.draw(canvas, &gi.camera);
            }
        }
        hover
    }
    fn quit(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) {
        if self.bud_data_left.is_some() {
            self.bud_data_left.as_ref().unwrap().borrow_mut().unselect();
        }
        if self.bud_data_right.is_some() {
            self.bud_data_right
                .as_ref()
                .unwrap()
                .borrow_mut()
                .unselect();
        }
    }
}
