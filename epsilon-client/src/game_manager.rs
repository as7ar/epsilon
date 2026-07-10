use crate::data;
use crate::data::get_user;
use epsilon_core::Player;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    base: Base<Node>,

    pub player: Player
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Self::Base>) -> Self {
        data::init();
        let user = get_user();
        let player = Player::new(user.clone());

        Self {
            base,

            player
        }
    }

    fn ready(&mut self) {
        self.base_mut()
            .add_to_group("game_manager");
    }
}

#[godot_api]
impl GameManager {
    #[func]
    fn get_player() {}
}
