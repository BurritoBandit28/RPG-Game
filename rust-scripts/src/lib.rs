mod npc;
mod player;
mod interactable;
mod button;
mod button_container;
mod LimboPlayerStats;
mod ButtonResult;

use godot::prelude::*;

struct RustLoaderExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustLoaderExtension {}
