use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::global::godot_print;
use godot::obj::{Base, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass, Vector2};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>
}

const SPEED: f32 = 300.0;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut direction = Vector2::ZERO;

        if input.is_action_pressed("move_left") {
            // godot_print!("move");
            direction.x -= 1.0;
        }
        if input.is_action_pressed("move_right") {
            // godot_print!("move");
            direction.x += 1.0;
        }
        if input.is_action_pressed("move_up") {
            // godot_print!("move");
            direction.y -= 1.0;
        }
        if input.is_action_pressed("move_down") {
            // godot_print!("move");
            direction.y += 1.0;
        }

        if direction != Vector2::ZERO {
            direction = direction.normalized();
        }

        // godot_print!("move");

        let velocity = direction * SPEED;

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        // godot_print!("{:?}", self.base().get_global_position());
    }
}