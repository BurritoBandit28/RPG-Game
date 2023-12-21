mod npc;
mod player;
mod interactable;
mod button;

use godot::prelude::*;

struct RustLoaderExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustLoaderExtension {}
