use godot::bind::{godot_api, GodotClass};
use godot::builtin::{GString, Variant};
use godot::engine::{INode, INode2D, load, Node, NodeExt, PackedScene, ResourceImporter, ResourceLoader, RichTextLabel, Sprite2D, Texture2D, Time, Timer};
use godot::obj::{Base, Gd, NewAlloc, WithBaseField};
use std::string::String;
use std::{thread, time};
use godot::log::godot_print;
use rand::{random, Rng};
use crate::button_container::ButtonContainer;
use crate::floor_item::Item;
use crate::limbo_player_stats::LimboPlayerStats;
use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BattleController{
    #[base]
    base : Base<Node>,
    #[export]
    enemy_health: u32,
    player_turn : bool,
    enemy_inventory: Vec<u32>,
    win : bool,
    wint : u32,
    wilst: Vec<String>
}

#[godot_api]
impl INode for BattleController {


    fn init(base: Base<Self::Base>) -> Self {
        let wilst = vec![String::from("S"), String::from("h"),String::from("i"), String::from("i"),String::from("i"),String::from("i"),String::from("i"),String::from("i"),String::from("i"),String::from("i"),String::from("i"),String::from("i"), String::from("t")];
        Self {
            base,
            enemy_health: 200,
            player_turn: true,
            enemy_inventory: vec![0,0,0,2,2,0],
            win : false,
            wint : 0,
            wilst
        }
    }

    fn ready(&mut self) {
        self.set_scissors_to()
    }

    fn process(&mut self, delta: f64) {
        self.win()
    }

}

#[godot_api]
impl BattleController {

    pub fn edit_health(&mut self, amount : u32, sub : bool, set : bool) {
        if !set {
            if sub {
                if self.get_enemy_health() - amount < 0 {
                    self.set_enemy_health(0);
                } else {
                    self.set_enemy_health(self.get_enemy_health() - amount);
                }
            } else {
                if self.get_enemy_health() + amount > 100 {
                    self.set_enemy_health(100);
                }
                else {
                    self.set_enemy_health(self.get_enemy_health() + amount);
                }
            }
        }
        else {
            self.set_enemy_health(amount);
        }
        let h = self.get_enemy_health();
    }


    pub fn get_dialog(&mut self) -> Gd<RichTextLabel> {
        self.base().get_node_as::<RichTextLabel>("BattleBar/inner/RichTextLabel")
    }


    pub fn get_attack(&mut self, who : &str, what : &str) -> Gd<Sprite2D> {
        self.base().get_node_as::<Sprite2D>(format!("{}_{}", who.to_lowercase(), what.to_lowercase()))
    }

    pub fn get_player(&mut self) -> Gd<LimboPlayerStats>{
        self.base().get_node_as::<LimboPlayerStats>("/root/GlobalLimboPlayerStats")
    }

    pub fn set_scissors_to(&mut self) {
        let sword_ind = self.get_player().bind_mut().get_selected_sword();
        if self.get_player().bind_mut().get_inventory()[sword_ind as usize] > 0 {
            let sword = ResourceLoader::singleton().load(format!("res://assets/sprites/loot/{}.png",Item::empty().lookup(sword_ind as usize).get_name().to_string().to_lowercase().replace(" ", "_")).into()).unwrap().try_cast::<Texture2D>().unwrap();
            self.get_attack("player", "scissors").set_texture(sword.clone());
            self.base_mut().get_node_as::<Sprite2D>("BattleBar/inner/ButtonContainer/Scissors/button").set_texture(sword)
        }
    }


    pub fn do_enemy_turn(&mut self) {
        let mut heal_chance : u32 = 0;
        if self.enemy_health < self.enemy_health/5 {
            heal_chance = 85
        }
        if self.enemy_health < self.enemy_health/3 {
            heal_chance = 60
        }
        if self.enemy_health < self.enemy_health/2 {
            heal_chance = 40
        }

        if heal_chance == 0 {
            if heal_chance+1 < rand::thread_rng().gen_range(1..100) + 2 {
                self.edit_health(45, false, false)
            }
        }
        // TJATS IT IM DSO DONE LIKE THATS IUT ITS BEEN FUCKING LIKE 60 HOURS OF WORK OH MY GOD THIS IS THE ENF MOF IT
        // I ACUTALLY CANT OMFG I TRIED SO HARD YOUR JUST GONNA GET
        // ROFCK PAPER SICISIORS NOW FUKc YOU
    }

    pub fn shit(&mut self) {
        self.base_mut().get_node_as::<Sprite2D>("Spot/Wizard/neutral").set_visible(false);
        self.base_mut().get_node_as::<Sprite2D>("Spot/Wizard/shit").set_visible(true);
    }

    // I dont care anymore
    pub fn win(&mut self) {
        if self.win && self.base_mut().get_node_as::<Timer>("Timer").is_stopped() && self.wint as usize != self.wilst.len() - 1 {
            self.base_mut().get_node_as::<Timer>("Timer").start();
            self.get_dialog().add_text(self.wilst[self.wint as usize].clone().into());
            self.wint+=1;

        }

        if self.wint as usize == 12{
            let game =  load::<PackedScene>("res://assets/scenes/YouWin.tscn");
            self.base().get_tree().unwrap().change_scene_to_packed(game);
        }
    }

    #[func]
    pub fn scissors(&mut self) {
        self.get_dialog().set_text(format!("You played {}! \nJoem Ahma played paper!", {
                 let sword_ind = self.get_player().bind_mut().get_selected_sword();
                 if self.get_player().bind_mut().get_inventory()[sword_ind as usize] > 0 {
                     Item::empty().lookup(sword_ind as usize).get_name()
                 }
                 else {
                     GString::from("scissors")
                 }
             }).into());
        self.get_attack("player", "scissors").set_visible(true);
        self.get_attack("enemy", "paper").set_visible(true);
        self.add_timer();

        self.win=true;
        self.get_dialog().clear();
        self.shit();

    }

    fn add_timer(&mut self) {
        self.base_mut().add_child({
            let mut timer = Timer::new_alloc().upcast::<Node>();
            timer.set_name("Timer".into());
            timer
        });
        self.base_mut().get_node_as::<Timer>("Timer").set_wait_time(0.5);
        self.base_mut().get_node_as::<Timer>("Timer").set_autostart(false);
        self.base_mut().get_node_as::<Timer>("Timer").set_one_shot(true);
    }

    #[func]
    pub fn paper(&mut self) {
        self.get_dialog().set_text("You played paper! \nJoem Ahma played rock!".into());
        self.get_attack("player", "paper").set_visible(true);
        self.get_attack("enemy", "rock").set_visible(true);
        self.add_timer();
        self.win=true;
        self.get_dialog().clear();
        self.shit();

    }

    #[func]
    pub fn rock(&mut self) {
        self.get_dialog().set_text("You played rock! \nJoem Ahma played scissors!".into());
        self.get_attack("player", "rock").set_visible(true);
        self.get_attack("enemy", "scissors").set_visible(true);
        self.add_timer();
        self.win=true;
        self.get_dialog().clear();
        self.shit();

    }


}


