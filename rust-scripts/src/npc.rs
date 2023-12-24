use std::fmt::Debug;
use std::thread::current;
use godot::engine::{IStaticBody2D, StaticBody2D, Node};
use godot::engine::utilities::str;
use godot::prelude::*;
use crate::interactable::Interactable;
use crate::player::Player;


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
    #[export]
    dialog : Array<VariantArray>,
    current_dialog : usize,
    current_line : usize
}

#[godot_api]
impl IStaticBody2D for NPC {
    fn init(sb: Base<StaticBody2D>) -> Self {
        Self {
            name: GString::default(),
            character: GString::default(),
            sb,
            dialog : Array::new(),
            current_line : 0usize,
            current_dialog : 0usize
        }
    }
}

impl Interactable for NPC {

}

#[godot_api]
impl NPC  {

    //#[func]
    pub fn interact(&mut self, player: &mut Player) {

        player.get_text().clear();
        player.get_name_tag_text().clear();

        if self.current_dialog >= self.dialog.len() {
            self.current_dialog-=1;
        }
        if self.current_line == self.dialog.get(self.current_dialog).len() {
            self.current_line = 0;
            self.current_dialog+=1;
            player.set_current_dialog_over(true);
           return;
        }
        else {
            player.set_current_dialog_over(false);
            player.get_text_box().set_visible(true);
            player.get_name_tag().set_visible(true);
        }


        godot_print!("{}", self.dialog.get(self.current_dialog).get(self.current_line));
        player.get_name_tag_text().add_text(GString::from(format!("{} the {}", self.name, self.character)));
        player.get_text().add_text(self.insert_variables(GString::from(self.dialog.get(self.current_dialog).get(self.current_line).stringify()), player));
        self.current_line +=1;



    }

    fn insert_variables(&mut self, text : GString, player : &mut Player) -> GString {
        let mut str : String = text.to_string();
        str = str.replace("{name}", self.name.to_string().as_str());
        str = str.replace("{character}", self.character.to_string().as_str());
        str = str.replace("{pname}", player.get_name().to_string().as_str());
        str = str.replace("{padj}", player.get_adjective().to_string().as_str());
        return GString::from(str);

    }

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

