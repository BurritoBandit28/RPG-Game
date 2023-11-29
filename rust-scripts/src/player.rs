
use godot::builtin::meta::GodotConvert;
use godot::engine::{
    IRigidBody2D, ISprite2D, IStaticBody2D, RigidBody2D,
};
use godot::prelude::*;
use std::fmt::Display;

use std::str::FromStr;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
struct Player {
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
    adjective : GString
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
            adjective : GString::from("Brave")
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
        if Input::is_action_pressed(&input, StringName::from_str("interact").unwrap()) {
            self.raycast();
            godot_print!("{}", self.get_player_name())
        }
        vel.normalized();
        self.rb.set_linear_velocity(vel)
    }
}
#[godot_api]
impl Player {
    #[func]
    fn raycast(&mut self) {
        self.rb.emit_signal(
            "raycast_signal".into(),
            &[Variant::from(self.facing.to_godot())],
        );
    }

    #[func]
    fn get_player_name(&mut self) -> GString {
        return GString::from(format!("{} the {}", self.name, self.adjective))
    }

    #[func]
    fn set_player_name(&mut self, nname : GString, nadjective : GString) {
        self.name = nname;
        self.adjective = nadjective
    }

    #[signal]
    fn raycast_signal(facing: StringName);
}

// add an enum for the facing direction compatible with GD script
pub enum Facing {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

impl GodotConvert for Facing {
    type Via = StringName;
}

impl ToGodot for Facing {
    fn to_godot(&self) -> Self::Via {
        return StringName::from(self.to_string());
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
