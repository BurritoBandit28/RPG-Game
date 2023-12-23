mod npc;
mod player;
mod interactable;
mod button;
mod button_container;

use godot::prelude::*;

struct RustLoaderExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustLoaderExtension {}
