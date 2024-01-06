use std::convert::Into;
use godot::bind::{godot_api, GodotClass};
use godot::builtin::{Array, GString, VariantArray};
use godot::engine::{Area2D, IArea2D, IStaticBody2D, Node2D, StaticBody2D};
use godot::obj::{Base, WithBaseField};
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct FloorItem {

    #[base]
    node: Base<Node2D>,
    #[export]
    index : u32,
    #[export]
    amount : u32

}



#[godot_api]
impl IArea2D for FloorItem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            node: base,
            index : 0,
            amount : 1
        }
    }
}

#[godot_api]
impl FloorItem {
    pub fn interact(&mut self, player: &mut Player) {

        player.get_text().clear();
        player.get_name_tag_text().clear();

        player.get_text().set_text(GString::from(format!("You picked up {} {}(s)", self.amount, Item::empty().lookup(self.index as usize).name)));
        player.get_text_box().set_visible(!player.get_current_dialog_over());
        if !player.get_current_dialog_over() {
            player.add_inventory(self.index as usize, self.amount);
            self.base_mut().queue_free();
        }
        let x = !player.get_current_dialog_over();
        player.set_current_dialog_over(x);
        player.get_name_tag().set_visible(false);
    }
}



#[derive(Debug)]
pub enum ItemType {
    HEAL,
    WEAPON,
    GOOD_POTION,
    BAD_POTION,
    REMEDY
}

impl Clone for ItemType {
    fn clone(&self) -> Self {
        match self {
            &ItemType::HEAL => {ItemType::HEAL}
            &ItemType::WEAPON => {ItemType::WEAPON}
            &ItemType::GOOD_POTION => {ItemType::GOOD_POTION}
            &ItemType::BAD_POTION => {ItemType::BAD_POTION}
            &ItemType::REMEDY => {ItemType::REMEDY}
        }
    }
}

impl Copy for ItemType {

}

#[derive(Debug)]
pub struct Item {
    itype : ItemType,
    heal_amount : u32,
    damage_amount : u32,
    name : GString
}

impl Item {

    pub fn lookup(&mut self, index : usize) -> Item {
        match index {
            1 => {Item {
                itype: ItemType::WEAPON,
                heal_amount: 0,
                damage_amount: 25,
                name: "Iron Sword".into(),
            }},
            2 => {Item {
                itype: ItemType::WEAPON,
                heal_amount: 0,
                damage_amount: 50,
                name: "Master Sword".into(),
            }},
            3 => {Item {
                itype: ItemType::GOOD_POTION,
                heal_amount: 45,
                damage_amount: 0,
                name: "Health Potion".into(),
            }},
            4 => {Item {
                itype: ItemType::BAD_POTION,
                heal_amount: 0,
                damage_amount: 16,
                name: "Poison Potion".into(),
            }},
            5 => {Item {
                itype: ItemType::REMEDY,
                heal_amount: 0,
                damage_amount: 0,
                name: "Poison Antidote Potion".into(),
            }}
            _ => {Item {
                itype: ItemType::WEAPON,
                heal_amount: 0,
                damage_amount: 10,
                name: "Wooden Sword".into(),
            }},
        }

    }


    pub fn empty() -> Self {
        Self {
            itype: ItemType::HEAL,
            heal_amount: 0,
            damage_amount: 0,
            name: "".into(),
        }
    }

    pub fn get_name(self) -> GString {
        self.name
    }
    pub fn get_heal_amount(self) -> u32 {
        self.heal_amount
    }
    pub fn get_damage_amount(self) -> u32 {
        self.damage_amount
    }
    pub fn get_type(self) -> ItemType {
        self.itype.clone()
    }

}

/*
impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            itype : self.get_type().clone(),
            heal_amount: self.get_heal_amount(),
            damage_amount: self.get_damage_amount(),
            name: self.get_name(),
        }
    }
}

impl Copy for Item {
}


 */
