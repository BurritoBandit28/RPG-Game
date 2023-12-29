use godot::bind::{godot_api, GodotClass};
use godot::builtin::{Array, GString, VariantArray};
use godot::engine::{IStaticBody2D, StaticBody2D};
use godot::obj::Base;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=StaticBody2D)]
pub struct FloorItem {

    #[base]
    sb: Base<StaticBody2D>,
    //todo

}



#[godot_api]
impl IStaticBody2D for FloorItem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            sb : base
        }
    }
}

#[godot_api]
impl FloorItem {
    pub fn interact(&mut self, player: &mut Player) {
        //todo
    }
}