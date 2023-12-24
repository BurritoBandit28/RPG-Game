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
    #[export]
    button_types : GString,
    #[base]
    sprite : Base<Area2D>,
    hovered : bool

}


#[godot_api]
impl IArea2D for UIButton {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            sprite : base,
            button_types : GString::from("play"),
            hovered : false
        }
    }



    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let btype = self.get_type();
        if self.get_hovered() && input.is_action_pressed(StringName::from_str("click").unwrap()) && self.get_interact_timer().is_stopped() {
            self.do_button_action(btype);
            self.get_interact_timer().start()
        }
    }



}


#[godot_api]
impl UIButton {

    pub fn do_button_action(&mut self, ButtonType : ButtonType) {

        // now where have I seen this before..?
        match ButtonType {
            ButtonType::PLAY => {
                //todo
                godot_print!("play");
                let game =  load::<PackedScene>("res://assets/scenes/LevelOne.tscn");
                self.sprite.tree().unwrap().change_scene_to_packed(game);
            }
            ButtonType::QUIT => {
                //todo
                godot_print!("quit")
            }
            ButtonType::OPTIONS => {
                //todo
                godot_print!("options")
            }
            ButtonType::SOURCE => {

                // writing ``let _ = ...`` is the convention for when ignoring return values in Rust
                let _ = that("https://github.com/BurritoBandit28/RPG-Game/tree/master/rust-scripts");
                // no idea if that will work on the web build or not guess we'll find out
            }
        }
    }

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

    pub fn get_type(&mut self) -> ButtonType {
        match self.button_types.to_string().to_lowercase().as_str() {
            "options" => {ButtonType::OPTIONS},
            "quit" => {ButtonType::QUIT},
            "source" => {ButtonType::SOURCE}
            _ => {ButtonType::PLAY}
        }
    }

}

pub enum ButtonType {
    PLAY,
    QUIT,
    OPTIONS,
    SOURCE
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ButtonType::OPTIONS => "Options".to_string(),
            ButtonType::QUIT => "Quit".to_string(),
            ButtonType::SOURCE => "Source".to_string(),
            _ => "Play".to_string(),
        };
        write!(f, "{}", str)
    }
}

