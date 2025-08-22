use std::{cell::RefCell, rc::Rc};

use sdl2::{mouse::MouseState, render::Canvas, video::Window};

use crate::game::{
    button::Button,
    game_info::GameInfo,
    game_object::{game_objects::GameObjectEnum, GameObject, SuperGameObject},
    game_state::StateInfo,
    menu::menu_state::menu_states::MenuStateHandler,
};

use super::{camera::Camera, collision_system::collisions::Collisions, input::Input};

pub type Object<'g> = dyn SuperGameObject<'g, Input = MenuStateHandler<'g>> + 'g;

pub struct SceneManager<'g> {
    pub object_list: Vec<Rc<RefCell<Object<'g>>>>,
    // ui_List: Vec<UIObject>,
}

impl<'g> SceneManager<'g> {
    pub fn new() -> Self {
        Self {
            object_list: Vec::new(),
        }
    }
    pub fn add(&mut self, obj: Rc<RefCell<Object<'g>>>) {
        self.object_list.push(obj);
    }
    pub fn remove(&mut self, index: usize) {
        self.object_list.remove(index);
    }
    pub fn remove_all(&mut self, indexes: &[usize], collisions: &mut Collisions) {
        for (i, index) in indexes.iter().enumerate() {
            // self.object_list
            //     .get(*index - i)
            //     .unwrap()
            //     .borrow_mut()
            //     .remove_collider(collisions);
            self.object_list.remove(*index - i);
        }
    }
    pub fn update(
        &mut self,
        delta_time: f32,
        collisions: &mut Collisions<'g>,
        gi: &mut GameInfo<'g>,
        si: &mut StateInfo<'g>,
    ) {
        let mut indexes = Vec::new();
        for (i, obj) in self.object_list.iter_mut().enumerate() {
            if !obj.borrow_mut().update(delta_time, collisions, gi, si) {
                indexes.push(i);
            }
        }
        self.remove_all(&indexes, collisions);
        collisions.remove_all(&indexes);
    }
    pub fn press(&mut self, gi: &mut GameInfo, msh: &mut MenuStateHandler<'g>) {
        for (i, obj) in self.object_list.iter_mut().enumerate() {
            obj.borrow_mut()
                .press(&gi.input.mouse_state, msh, Some(&gi.camera));
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>, camera: &mut Camera) {
        for obj in self.object_list.iter() {
            // obj.borrow().draw(canvas, camera);
            // let obj = &obj.borrow();
            GameObject::draw(&*(obj.borrow()), canvas, camera);
        }
    }

    // void SceneManager::uiadd(UIObject* obj){
    //     uiList.push_back(obj);
    // }

    // void SceneManager::uiremoveDelete(UIObject* obj){
    //     uiList.erase(std::find(uiList.begin(), uiList.end(), obj));
    //     delete obj;
    // }
}
