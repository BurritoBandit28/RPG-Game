mod player;
mod npc;

use godot::prelude::*;

struct RustLoaderExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustLoaderExtension {}