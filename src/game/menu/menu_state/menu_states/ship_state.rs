use std::{cell::RefCell, rc::Rc};

use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::game::{
    button::{Button, MenuButton},
    game_info::GameInfo,
    game_object::game_objects::ship::{Ship, ShipData},
    menu::menu_state::{menu_states::MenuStateEnum, MenuState},
};

pub struct ShipState<'g> {
    buttons: Vec<MenuButton<Rc<RefCell<ShipData<'g>>>>>,
    ship_data: Option<Rc<RefCell<ShipData<'g>>>>,
}

impl<'g> ShipState<'g> {
    pub fn new(gi: &mut GameInfo<'g>) -> Self {
        let mut buttons = Vec::new();
        buttons.push(MenuButton::new(
            Rect::new(0, 50, 100, 100),
            "Right",
            Box::new(|ship_data: &mut Rc<RefCell<ShipData<'g>>>| {
                ship_data.borrow_mut().turn(1.0);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(0, 150, 100, 100),
            "Left",
            Box::new(|ship_data| {
                ship_data.borrow_mut().turn(-1.0);
            }),
        ));

        buttons.push(MenuButton::new(
            Rect::new(0, 250, 100, 100),
            "Gas",
            Box::new(|ship_data| {
                ship_data.borrow_mut().gas(1.0);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(0, 350, 100, 100),
            "Brake",
            Box::new(|ship_data| {
                ship_data.borrow_mut().gas(-1.0);
            }),
        ));
        buttons.push(MenuButton::new(
            Rect::new(0, 450, 100, 100),
            "GO!",
            Box::new(|ship_data| {
                ship_data.borrow_mut().go();
            }),
        ));
        Self {
            buttons,
            ship_data: None,
        }
    }
}

impl<'g> MenuState<'g> for ShipState<'g> {
    fn load(
        &mut self,
        gi: &mut GameInfo<'g>,
        delta_time: f32,
        menu_state_enum: &MenuStateEnum<'g>,
    ) {
        match menu_state_enum {
            MenuStateEnum::Ship(ship_data) => {
                if let Some(ship_data) = ship_data {
                    if self.ship_data.is_some() {
                        self.ship_data.as_ref().unwrap().borrow_mut().unselect();
                    }
                    self.ship_data = Some(Rc::clone(ship_data));
                    self.ship_data.as_ref().unwrap().borrow_mut().select();
                }
            }
            _ => unreachable!(),
        }
    }

    fn run(&mut self, gi: &mut GameInfo<'g>, delta_time: f32, canvas: &mut Canvas<Window>) -> bool {
        let mut hover = false;
        if let Some(ship_data) = &mut self.ship_data {
            for button in self.buttons.iter_mut() {
                if button
                    .press(&gi.input.mouse_state, ship_data, Some(&gi.camera))
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
        if self.ship_data.is_some() {
            self.ship_data.as_ref().unwrap().borrow_mut().unselect();
        }
    }
}
