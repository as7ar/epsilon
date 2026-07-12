mod scene;
mod player;
pub mod data;
pub mod utils;
mod ui;
mod game_manager;

use godot::prelude::*;

struct Epsilon;

#[gdextension]
unsafe impl ExtensionLibrary for Epsilon {
    fn on_stage_init(stage: InitStage) {
        if stage==InitStage::Core {
        }
    }
}