#![allow(warnings)]
// ^^ bad practice but idc rn lol

mod npc;
mod player;
mod interactable;
mod button;
mod button_container;
mod limbo_player_stats;
mod button_result;
mod floor_item;
mod battle_controller;

use godot::prelude::*;
struct RustLoaderExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustLoaderExtension {}
