use godot::classes::{Control, IControl, InputEvent, InputEventKey};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct StartScene {
    base: Base<Control>,
    started: bool
}

#[godot_api]
impl IControl for StartScene {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            started: false,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.started {
            return;
        }

        let key = event.clone().try_cast::<InputEventKey>();

        if key.is_err() {
            return;
        }

        // godot_print!("{}", key.clone().unwrap().as_text());

        if key.unwrap().is_pressed() {
            self.started=true;
            self.base().get_tree().change_scene_to_file("res://scene/lobby_scene.tscn");
        }
    }
}