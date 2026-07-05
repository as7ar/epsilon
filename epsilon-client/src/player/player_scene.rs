use godot::classes::{CharacterBody2D, ICharacterBody2D, Input, ProjectSettings};
use godot::global::Key;
use godot::obj::{Base, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use epsilon_core::CoolDown;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,

    player_face: i16, // 1: right, 0: left

    can_double_jump: bool,

    pub(crate) dash_cool: CoolDown,
    is_dashing: bool,
    dash_time: f32,

    coyote_time: f32,
}

const SPEED: f32 = 300.0;
const JUMP_VELOCITY: f32 = -450.0;
const COYOTE_DURATION: f32 = 0.25;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,

            player_face: 1,

            can_double_jump: true,

            dash_cool: CoolDown::new(1700.),
            is_dashing: false,
            dash_time: 0.0,

            coyote_time: 0.0,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        let gravity = ProjectSettings::singleton()
            .get_setting("physics/2d/default_gravity")
            .to::<f32>();

        self.dash_cool.tick();

        if self.is_dashing {
            self.dash_time -= delta as f32;

            let d = if self.player_face == 0 { -1.0 } else { 1.0 };
            velocity.x = SPEED * 5.5 * d;

            if self.dash_time <= 0.0 {
                self.is_dashing = false;
            }
        } else {
            let mut direction = 0.0;

            if input.is_action_pressed("move_left") {
                direction -= 1.0;
                self.player_face = 0;
            }

            if input.is_action_pressed("move_right") {
                direction += 1.0;
                self.player_face = 1;
            }

            velocity.x = direction * SPEED;
        }

        if self.base().is_on_floor() {
            velocity.y = 0.0;
            self.coyote_time = COYOTE_DURATION;
            self.can_double_jump = true;
        } else {
            velocity.y += gravity * delta as f32;
            self.coyote_time -= delta as f32;
        }

        if input.is_key_pressed(Key::K) && self.dash_cool.try_use() {
            self.is_dashing = true;
            self.dash_time = 0.15;
        }

        if input.is_action_just_pressed("jump") {
            if self.coyote_time > 0.0 {
                velocity.y = JUMP_VELOCITY;
                self.coyote_time = 0.0;
            } else if self.can_double_jump {
                velocity.y = JUMP_VELOCITY;
                self.can_double_jump = false;
            }
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}