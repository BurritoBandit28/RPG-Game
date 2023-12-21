
use godot::builtin::meta::GodotConvert;
use godot::engine::{IRigidBody2D, ISprite2D, IStaticBody2D, RayCast2D, RichTextLabel, RigidBody2D, Sprite2D, Timer};
use godot::prelude::*;
use std::fmt::Display;

use std::str::FromStr;
use crate::interactable::Interactable;
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
    current_dialog_over: bool
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
            current_dialog_over : true

        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut vel = Vector2::new(0.0, 0.0);
        let val: f32 = {
            if Input::is_action_pressed(&input, StringName::from_str("shift").unwrap()) {
                self.sprint
            } else {
                self.speed
            }
        };

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
}
#[godot_api]
impl Player {

    pub fn get_interact_timer(&mut self) -> Gd<Timer> {
        return self.rb.get_node_as::<Timer>("InteractTimer");
    }

    pub fn get_text(&mut self) -> Gd<RichTextLabel> {
        return self.rb.get_node_as::<RichTextLabel>("Camera2D/TextBox/Text");
    }
    pub fn get_text_box(&mut self) -> Gd<Sprite2D> {
        return self.rb.get_node_as::<Sprite2D>("Camera2D/TextBox");
    }

    pub fn set_current_dialog_over(&mut self, tf : bool) {
        self.current_dialog_over =tf;
        self.get_text_box().set_visible(!tf);
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

            let mut hit = &ray.collider().unwrap();


            // not the sleekest implementation... but it works and thats the important part
            match hit.get_class().to_string().as_str() {
                "NPC" => { hit.clone().try_cast::<NPC>().unwrap().bind_mut().interact(self); }
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
