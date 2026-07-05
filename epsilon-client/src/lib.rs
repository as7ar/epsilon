mod scene;
mod player;
mod data;
mod utils;

use godot::prelude::*;

struct Epsilon;

#[gdextension]
unsafe impl ExtensionLibrary for Epsilon {}
