
use godot::builtin::meta::GodotConvert;
use godot::engine::{Area2D, Engine, IRigidBody2D, ISprite2D, IStaticBody2D, RayCast2D, RichTextLabel, RigidBody2D, Sprite2D, Timer};
use godot::prelude::*;
use std::fmt::Display;

use std::str::FromStr;
use crate::floor_item::{FloorItem, Item};
use crate::interactable::Interactable;
use crate::limbo_player_stats::LimboPlayerStats;
use crate::npc::NPC;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Player {
    #[export]
    speed: f32,
    #[export]
    sprint: f32,
    #[base]
    rb: Base<RigidBody2D>,
    facing: Facing,
    #[export]
    name : GString,
    #[export]
    adjective : GString,
    current_dialog_over: bool,
    #[export]
    health : u32,
    inventory : Vec<u32>,
    inventory_open : bool,
    selected_sword : u32
}

#[godot_api]
impl IRigidBody2D for Player {
    fn init(rb: Base<RigidBody2D>) -> Self {
        godot_print!("Yea we gaming");
        Self {
            speed: 50.0,
            sprint: 95.0,
            rb,
            facing: Facing::RIGHT,
            name : GString::from("Jilly Tismond"),
            adjective : GString::from("Brave"),
            current_dialog_over : true,
            health : 100,
            inventory : vec![0,0,0,0,0,0],
            inventory_open : false,
            selected_sword : 0
        }
    }

    fn physics_process(&mut self, _ : f64) {
        let input = Input::singleton();
        let mut vel = Vector2::new(0.0, 0.0);
        let val: f32 = {
            if Input::is_action_pressed(&input, StringName::from_str("shift").unwrap()) {
                self.sprint
            } else {
                self.speed
            }
        };
        if !self.inventory_open {
            if self.current_dialog_over {
                if Input::is_action_pressed(&input, StringName::from_str("a").unwrap()) {
                    vel.x -= val;
                    (*self).facing = Facing::LEFT;
                }
                if Input::is_action_pressed(&input, StringName::from_str("d").unwrap()) {
                    vel.x += val;
                    (*self).facing = Facing::RIGHT;
                }
                if Input::is_action_pressed(&input, StringName::from_str("s").unwrap()) {
                    vel.y += val;
                    (*self).facing = Facing::DOWN;
                }
                if Input::is_action_pressed(&input, StringName::from_str("w").unwrap()) {
                    vel.y -= val;
                    (*self).facing = Facing::UP;
                }
                vel.normalized();
                self.rb.set_linear_velocity(vel);
                let mut ray = &mut self.rb.get_node_as::<RayCast2D>("RayCast2D");
                ray.set_rotation_degrees(self.facing.to_rot());
            }
            if Input::is_action_pressed(&input, StringName::from_str("interact").unwrap()) && self.get_interact_timer().is_stopped() {
                self.raycast();
                //godot_print!("{}", self.get_player_name())
            }
        }
        if Input::is_action_pressed(&input, StringName::from_str("inventory").unwrap() ) && self.get_interact_timer().is_stopped() && self.current_dialog_over {
            self.get_interact_timer().start();
            self.inventory_open = !self.inventory_open;

            if self.inventory_open {

                let inventory_screen = load::<PackedScene>("res://assets/prefabs/player/inventory_screen.tscn").instantiate();
                self.get_camera().add_child(inventory_screen.unwrap());

                for x in 0..self.inventory.len() {
                    godot_print!("{} x{}", Item::empty().lookup(x).get_name(), self.inventory[x])
                }
            }
            else {
                self.get_inventory_screen().free();
            }
        }
    }

    fn ready(&mut self) {
        let mut binding = self.get_limbo_stats();
        let mut limbo = binding.bind_mut();
        self.name = limbo.get_name();
        self.adjective = limbo.get_adjective();
        self.edit_health(limbo.get_health(), false, true);
    }
}


#[godot_api]
impl Player {



    pub fn add_inventory(&mut self, index : usize, amount : u32) {
        self.inventory[index]+=amount
    }
    pub fn take_inventory(&mut self, index : usize, amount : u32) -> Result<(), ()> {
        if self.inventory[index] < amount {
            Err(())
        }
        else {
            self.inventory[index] -=amount;
            Ok(())
        }
    }

    #[func]
    pub fn set_selected_sword(&mut self, ind : u32) {
        self.selected_sword = ind
    }

    fn get_limbo_stats(&mut self) -> Gd<LimboPlayerStats> {
        return self.rb.get_node_as::<LimboPlayerStats>("/root/GlobalLimboPlayerStats")
    }
    #[func]
    pub fn save_stats(&mut self) {
        let mut binding = self.get_limbo_stats();
        let mut limbo = binding.bind_mut();
        limbo.set_health(self.health);
    }

    #[func]
    pub fn area_entered(&mut self, mut area : Gd<Area2D>) {
        if area.z_index() > self.rb.z_index() || area.z_index() == self.rb.z_index() {
            self.rb.set_z_index(area.z_index()+1);
        }
    }

    #[func]
    pub fn get_selected_sword(&mut self) -> u32 {
        return self.selected_sword
    }

    #[func]
    pub fn edit_health(&mut self, amount : u32, sub : bool, set : bool) {
        if !set {
            if sub {
                if self.get_health() - amount < 0 {
                    self.set_health(0);
                } else {
                    self.set_health(self.get_health() - amount);
                }
            } else {
                if self.get_health() + amount > 100 {
                    self.set_health(100);
                }
                else {
                    self.set_health(self.get_health() + amount);
                }
            }
        }
        else {
            self.set_health(amount);
        }
        let h = self.get_health();
        self.rb.emit_signal("health_changed".into(), &[Variant::from(h)]);
    }

    #[signal]
    pub fn health_changed(&mut self, health : u32);

    #[func]
    pub fn body_entered(&mut self, mut node : Gd<Node2D>) {

        if node.get_class() == GString::from("Player") {
            return;
        }
        if node.global_transform().origin.y < self.rb.global_transform().origin.y {
            if node.z_index() > self.rb.z_index() {
                let z = node.z_index();
                node.set_z_index(self.rb.z_index());
                self.rb.set_z_index(z);
            }
            if node.z_index() == self.rb.z_index() {
                let x= self.rb.z_index()+1;
                self.rb.set_z_index(x);
            }
        }
        else if node.global_transform().origin.y > self.rb.global_transform().origin.y{
            if node.z_index() < self.rb.z_index() {
                let z = self.rb.z_index();
                self.rb.set_z_index(node.z_index());
                node.set_z_index(z);
            }
            if node.z_index() == self.rb.z_index() {
                let x= node.z_index()+1;
                node.set_z_index(x);
            }
        }
    }

    pub fn get_interact_timer(&mut self) -> Gd<Timer> {
        return self.rb.get_node_as::<Timer>("InteractTimer");
    }

    pub fn get_camera(&mut self) -> Gd<Camera2D> {
        return self.rb.get_node_as::<Camera2D>("Camera2D");
    }

    pub fn get_text(&mut self) -> Gd<RichTextLabel> {
        return self.rb.get_node_as::<RichTextLabel>("Camera2D/TextBox/Text");
    }
    pub fn get_text_box(&mut self) -> Gd<Sprite2D> {
        return self.rb.get_node_as::<Sprite2D>("Camera2D/TextBox");
    }

    pub fn get_name_tag(&mut self) -> Gd<Sprite2D> {
        return self.rb.get_node_as::<Sprite2D>("Camera2D/NameTag");
    }

    pub fn get_name_tag_text(&mut self) ->Gd<RichTextLabel> {
        return self.rb.get_node_as::<RichTextLabel>("Camera2D/NameTag/Text");
    }
    pub fn get_inventory_screen(&mut self) ->Gd<Node2D> {
        return self.rb.get_node_as::<Node2D>("Camera2D/InventoryScreen");
    }

    #[func]
    pub fn get_item_count(&mut self, ind : u32) -> u32 {
        self.inventory[ind as usize]
    }

    pub fn set_current_dialog_over(&mut self, tf : bool) {
        self.current_dialog_over =tf;
        self.get_text_box().set_visible(!tf);
        self.get_name_tag().set_visible(!tf);
    }
    pub fn get_current_dialog_over(&mut self) -> bool {
        self.current_dialog_over
    }


    fn raycast(&mut self) {
        self.get_interact_timer().start();
        if self.get_text_box().is_visible() {
            if self.current_dialog_over {
                self.get_text_box().set_visible(false);
                self.get_text().set_text(GString::from(""));
                return;
            }
        }
        let mut ray = &mut self.rb.get_node_as::<RayCast2D>("RayCast2D");

        if ray.is_colliding() {

            let mut hit = &ray.collider().unwrap().cast::<Node>().parent().unwrap();

            // not the sleekest implementation... but it works and thats the important part
            match hit.clone().get_class().to_string().as_str() {
                "NPC" => {
                    hit.clone()
                        .try_cast::<NPC>()
                        .unwrap().bind_mut()
                        .interact(self); }

                "FloorItem" => {
                    hit.clone()
                        .try_cast::<FloorItem>()
                        .unwrap().bind_mut()
                        .interact(self); }

                _ => { godot_print!("uhm") }

            }


        }
        // without this call the memory goes all "oopsie I did a fucky >~<"
        //ray.queue_free();
        // actually nvm its handled by godot if I call that it actually fucks up more lol


    }

    /*
    fn do_interact(&mut self, mut obj : impl Interactable) {
        obj.interact(self)
    }
     */

    #[func]
    fn get_player_name(&mut self) -> GString {
        return GString::from(format!("{} the {}", self.name, self.adjective))
    }

    #[func]
    fn set_player_name(&mut self, nname : GString, nadjective : GString) {
        self.name = nname;
        self.adjective = nadjective
    }

}


// add an enum for the facing direction compatible with GD script
pub enum Facing {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl GodotConvert for Facing {
    type Via = GString;
}

impl ToGodot for Facing {
    fn to_godot(&self) -> Self::Via {
        return GString::from(self.to_string());
    }
}

impl Facing {
    fn to_rot(&mut self) -> f32 {
        return match self {
            Facing::LEFT => 90.0,
            Facing::UP => 180.0,
            Facing::DOWN => 0.0,
            _ => -90.0
        }
    }
}

impl Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Facing::LEFT => "LEFT".to_string(),
            Facing::UP => "UP".to_string(),
            Facing::DOWN => "DOWN".to_string(),
            _ => "RIGHT".to_string(),
        };
        write!(f, "{}", str)
    }
}
