use godot::engine::{IStaticBody2D, StaticBody2D};
use godot::prelude::*;


#[derive(GodotClass)]
// TODO - have npc + future interactable elements extend a common custom class, and not StaticBody2D
#[class(base=StaticBody2D)]
pub struct NPC {
    #[export]
    name: GString,
    #[export]
    character: GString,
    #[base]
    sb: Base<StaticBody2D>,
}

#[godot_api]
impl IStaticBody2D for NPC {
    fn init(sb: Base<StaticBody2D>) -> Self {
        Self {
            name: GString::default(),
            character: GString::default(),
            sb,
        }
    }
}

#[godot_api]
impl NPC {
    /*
       #[func]
       fn get_name(&mut self) -> StringName {
           return self.name
       }

       #[func]
       fn get_char_name(&mut self) -> StringName {
           return self.character
       }

    */
}
