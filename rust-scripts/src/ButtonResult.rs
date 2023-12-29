// little utility for changing scenes


use godot::engine::{INode, load, Node, PackedScene};
use godot::log::godot_print;
use godot::prelude::{Base, godot_api, GodotClass};
use open::that;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ButtonResult {
    #[base]
    base : Base<Node>
}

#[godot_api]
impl INode for ButtonResult {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

#[godot_api]
impl ButtonResult {
    #[func]
    pub fn load_scene_one(&mut self) {
        let game =  load::<PackedScene>("res://assets/scenes/LevelOne.tscn");
        self.base.tree().unwrap().change_scene_to_packed(game);
    }
    #[func]
    pub fn play(&mut self) {
        let game =  load::<PackedScene>("res://assets/scenes/ChooseName.tscn");
        self.base.tree().unwrap().change_scene_to_packed(game);
    }

    #[func]
    pub fn options(&mut self) {
        godot_print!("options")
    }

    #[func]
    pub fn quit(&mut self) {
        godot_print!("quit")
    }

    #[func]
    pub fn source(&mut self) {
        let _ = that("https://github.com/BurritoBandit28/RPG-Game/tree/master/rust-scripts");
    }
}