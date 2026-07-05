mod scene;
mod player;
pub mod data;
pub mod utils;

use godot::prelude::*;

struct Epsilon;

#[gdextension]
unsafe impl ExtensionLibrary for Epsilon {}
