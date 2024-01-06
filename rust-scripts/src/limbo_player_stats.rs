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
    inventory : Vec<u32>,
    selected_sword : u32

}

#[godot_api]
impl INode for LimboPlayerStats {

    // init function defining defaults for this class
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            name : GString::from("Jilly Tismond"),
            adjective : GString::from("Brave"),
            health : 100,
            inventory : vec![0,0,0,0,0,0],
            selected_sword : 0
        }
    }

}

#[godot_api]
impl LimboPlayerStats {

    pub fn set_inventory(&mut self, inv : Vec<u32>) {
        self.inventory = inv
    }
    pub fn get_inventory(&mut self) -> Vec<u32> {
        self.inventory.clone()
    }

    #[func]
    pub fn set_selected_sword(&mut self, sel: u32) {
        self.selected_sword = sel
    }

    #[func]
    pub fn get_selected_sword(&mut self) -> u32 {
        self.selected_sword
    }
}