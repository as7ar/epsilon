use crate::data::{get_user, PlayerStatus};
use epsilon_core::{CoolDown, Player};
use godot::classes::{AnimatedSprite2D, CharacterBody2D, ICharacterBody2D, Input, ProjectSettings};
use godot::global::Key;
use godot::obj::{Base, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass, Vector2};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerExt {
    base: Base<CharacterBody2D>,

    player: Player,

    player_face: i16,
    player_status: PlayerStatus,

    can_double_jump: bool,

    pub(crate) dash_cool: CoolDown,
    dash_time: f32,

    coyote_time: f32,
}

const SPEED: f32 = 300.0;
const DASH_SPEED: f32 = SPEED * 5.5;
const JUMP_VELOCITY: f32 = -450.0;
const COYOTE_DURATION: f32 = 0.25;
const DASH_DURATION: f32 = 0.15;

#[godot_api]
impl ICharacterBody2D for PlayerExt {
    fn init(base: Base<Self::Base>) -> Self {
        let player = Player::new(get_user());

        Self {
            base,

            player,

            player_face: 1,
            player_status: PlayerStatus::Standing,

            can_double_jump: true,

            dash_cool: CoolDown::new(1700.0),
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

        let x = self.base().get_position().x;
        let y = self.base().get_position().y;

        self.dash_cool.tick();

        // ================================[ Movements ]================================

        if self.player_status == PlayerStatus::Dash {
            self.dash_time -= delta as f32;

            let dir = if self.player_face == 0 { -1.0 } else { 1.0 };
            velocity.x = DASH_SPEED * dir;

            if self.dash_time <= 0.0 {
                self.player_status = PlayerStatus::Standing;
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

            if input.is_key_pressed(Key::K) && self.dash_cool.try_use() {
                self.player_status = PlayerStatus::Dash;
                self.dash_time = DASH_DURATION;
            }
        }

        if self.base().is_on_floor() {
            velocity.y = 0.0;
            self.coyote_time = COYOTE_DURATION;
            self.can_double_jump = true;
        } else {
            velocity.y += gravity * delta as f32;
            self.coyote_time -= delta as f32;
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

        if self.player_status != PlayerStatus::Dash {
            if !self.base().is_on_floor() {
                self.player_status = PlayerStatus::Jumping;
            } else if velocity.x.abs() > 0.0 {
                self.player_status = PlayerStatus::Running;
            } else {
                self.player_status = PlayerStatus::Standing;
            }
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        // ================================[ Animation ]================================

        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.set_flip_h(self.player_face == 0);

        let anim = match self.player_status {
            PlayerStatus::Standing => "standing",
            // PlayerStatus::Running => "run",
            // PlayerStatus::Jumping => "jump",
            // PlayerStatus::Dash => "dash",
            // PlayerStatus::Attacking => "attack",
            // PlayerStatus::Hit => "hit",

            _ => { "standing" }
        };

        if sprite.get_animation().to_string() != anim {
            sprite.play_ex().name(anim).done();
        }

        // ================================[ Falling Death ]================================

        if y >= 1200. {
            self.base_mut().set_position(Vector2 {
                x: 0.,
                y: -50.
            } )
        }

        // ================================[Debug]================================
        // godot_print!("x: {x}, y: {y}");
    }
}