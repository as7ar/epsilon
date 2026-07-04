mod scene;
mod player;
mod data;

use godot::prelude::*;

struct Epsilon;

#[gdextension]
unsafe impl ExtensionLibrary for Epsilon {}
