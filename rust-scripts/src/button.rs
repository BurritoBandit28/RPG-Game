use std::fmt::{Display, Formatter};
use std::str::FromStr;
use godot::bind::{godot_api, GodotClass};
use godot::bind::property::PropertyHintInfo;
use godot::builtin::{GString, StringName};
use godot::builtin::meta::ClassName;
use godot::engine::{Area2D, CollisionObject2D, IArea2D, ICollisionObject2D, Input, ISprite2D, NodeExt, Sprite2D};
use godot::init::InitLevel;
use godot::log::godot_print;
use godot::obj::{Base, GodotClass};
use godot::prelude::{Export, Gd, Property};
use godot::private::You_forgot_the_attribute__godot_api;
use crate::player::Facing;

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
        if self.get_hovered() && input.is_action_pressed(StringName::from_str("click").unwrap()) {
            self.do_button_action(btype)
        }
    }



}

#[godot_api]
impl UIButton {

    pub fn do_button_action(&mut self, ButtonType : ButtonType) {
        match ButtonType {
            ButtonType::PLAY => {
                //todo
            }
            ButtonType::QUIT => {
                //todo
            }
            ButtonType::OPTIONS => {
                //todo
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

    fn get_hovered_sprite(&mut self) -> Gd<Sprite2D> {
        self.sprite.get_node_as::<Sprite2D>("hovered")
    }
    pub fn set_hovered(&mut self, hovered : bool) {
        self.hovered = hovered;
        self.get_hovered_sprite().set_visible(hovered);
    }

    pub fn get_type(&mut self) -> ButtonType {
        match self.button_types.to_string().to_lowercase().as_str() {
            "options" => {ButtonType::OPTIONS},
            "quit" => {ButtonType::QUIT},
            _ => {ButtonType::PLAY}
        }
    }

}

pub enum ButtonType {
    PLAY,
    QUIT,
    OPTIONS
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ButtonType::OPTIONS => "Options".to_string(),
            ButtonType::QUIT => "Quit".to_string(),
            _ => "Play".to_string(),
        };
        write!(f, "{}", str)
    }
}

