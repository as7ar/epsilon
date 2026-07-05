use godot::classes::{CharacterBody2D, ICharacterBody2D, Input, ProjectSettings};
use godot::obj::{Base, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass, Vector2};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    can_double_jump: bool,
}

const SPEED: f32 = 300.0;
const JUMP_VELOCITY: f32 = -450.0;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            can_double_jump: true
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to::<f32>();

        if !self.base().is_on_floor() {
            velocity.y += gravity * delta as f32;
        }

        let mut direction = 0.0;

        if input.is_action_pressed("move_left") {
            direction -= 1.0;
        }

        if input.is_action_pressed("move_right") {
            direction += 1.0;
        }

        velocity.x = direction * SPEED;

        if input.is_action_just_pressed("jump") {
            match self.base().is_on_floor() {
                false => if self.can_double_jump {
                    velocity.y = JUMP_VELOCITY
                }
                true => velocity.y = JUMP_VELOCITY
            }
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}