use std::fmt::{Display, Formatter};
use std::str::FromStr;
use godot::bind::{godot_api, GodotClass};
use godot::bind::property::PropertyHintInfo;
use godot::builtin::{GString, StringName};
use godot::builtin::meta::{ClassName, GodotConvert};
use godot::engine::{Area2D, CollisionObject2D, IArea2D, ICollisionObject2D, Node, Input, ISprite2D, NodeExt, Sprite2D, Timer, PackedScene, SceneTree};
use godot::init::InitLevel;
use godot::log::godot_print;
use godot::obj::{Base, GodotClass};
use godot::prelude::{Export, FromGodot, Gd, load, Property};
use godot::private::You_forgot_the_attribute__godot_api;
use crate::button_container::ButtonContainer;
use crate::player::Facing;
use open::that;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct UIButton {
    #[base]
    sprite : Base<Area2D>,
    hovered : bool

}


#[godot_api]
impl IArea2D for UIButton {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            sprite : base,
            hovered : false
        }
    }



    fn process(&mut self, _ : f64) {
        let input = Input::singleton();
        if self.get_hovered() && input.is_action_pressed(StringName::from_str("click").unwrap()) && self.get_interact_timer().is_stopped() {
            self.do_button_action();
            self.get_interact_timer().start()
        }
    }



}


#[godot_api]
impl UIButton {

    #[func]
    pub fn do_button_action(&mut self) {
        self.sprite.emit_signal("button_pressed".into(), &[]);
    }

    #[signal]
    fn button_pressed(&mut self);

    #[func]
    fn mouse_enter(&mut self) {
        self.set_hovered(true)
    }

    #[func]
    fn mouse_exit(&mut self) {
        self.set_hovered(false)
    }

    pub fn get_hovered(&mut self) -> bool {
        self.hovered
    }

    pub fn get_interact_timer(&mut self) -> Gd<Timer> {
        self.sprite.parent().unwrap().cast::<ButtonContainer>().bind_mut().get_interact_timer()
    }

    fn get_hovered_sprite(&mut self) -> Gd<Sprite2D> {
        self.sprite.get_node_as::<Sprite2D>("hovered")
    }


    #[func]
    pub fn set_hovered(&mut self, hovered : bool) {
        self.hovered = hovered;
        self.get_hovered_sprite().set_visible(hovered);
    }


}