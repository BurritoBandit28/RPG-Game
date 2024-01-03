use std::fmt::Debug;
use std::io::read_to_string;
use std::thread::current;
use godot::engine::{IStaticBody2D, StaticBody2D, Node};
use godot::engine::utilities::str;
use godot::prelude::*;
use crate::floor_item::Item;
use crate::interactable::Interactable;
use crate::player::Player;


#[derive(GodotClass)]
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
        let mut upper = false;
        let mut str : String = text.to_string();
        str = str.replace("{name}", self.name.to_string().as_str());
        str = str.replace("{character}", self.character.to_string().as_str());
        str = str.replace("{pname}", player.get_name().to_string().as_str());
        str = str.replace("{padj}", player.get_adjective().to_string().as_str());
        if str.contains("{upper}") {upper = true}
        str = str.replace(" {upper}", "");
        let mut ls = str.split(" ");
        let mut ret_str : String = String::new();

        for mut x in ls {

            if x.to_string().chars().next().unwrap() == '[' {

                let ind = x.to_string().chars().nth(1usize).unwrap().to_string().parse::<usize>().unwrap();
                let amt = x.to_string().chars().nth(3usize).unwrap().to_string().parse::<u32>().unwrap();
                player.add_inventory(ind, amt);
                ret_str.push_str(format!("{} {}(s)",amt,Item::empty().lookup(ind).get_name().to_string()).as_str())
            }
            else {
                ret_str.push_str(x)
            }
            ret_str.push(' ');
        }

        if upper {
            return GString::from(ret_str.to_uppercase());
        }
        else {
            return GString::from(ret_str);
        }


    }

    /*


    let mut istr = "";
            if x.get(3usize).unwrap() == "]" {
                let ind = x.get(0usize) as usize;
                let amt = x.get(2usize) as u32;
                player.add_inventory(ind, amt);
                x = x.trim_matches({
                    [ind.to_string().chars()[0], ',', amt.to_string().chars()[0], ']']
                });
                istr = Item::empty().lookup(ind).get_name().to_string().as_str();
            }
            ret_str.push_str(istr);
            ret_str.push_str(x)


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

