// this file is used to store player stats and values inbetween scenes


use godot::bind::{godot_api, GodotClass};
use godot::builtin::{Array, GString, VariantArray};
use godot::engine::{INode, Node, NodeExt, RichTextLabel};
use godot::obj::{Base, Gd};
use crate::button_container::ButtonContainer;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LimboPlayerStats{
    #[base]
    base : Base<Node>,
    #[export]
    name : GString,
    #[export]
    adjective : GString,
    #[export]
    health : u32,


}

#[godot_api]
impl INode for LimboPlayerStats {

    // init function defining defaults for this class
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            name : GString::from("Jilly Tismond"),
            adjective : GString::from("Brave"),
            health : 100
        }
    }

}

#[godot_api]
impl LimboPlayerStats {


}