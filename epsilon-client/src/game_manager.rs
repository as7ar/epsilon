use std::fs;
use std::path::Path;
use crate::data;
use crate::data::get_user;
use crate::player::PlayerExt;
use epsilon_core::{Player, User};
use godot::classes::Engine;
use godot::prelude::*;
use epsilon_core::item::{Item, ItemRegistry};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    base: Base<Node>,

    pub user: User,
    pub item_registry: ItemRegistry
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Self::Base>) -> Self {
        data::init();
        let user = get_user();
        let mut item_registry = ItemRegistry::new();

        let path = Path::new("/assets/items");
        for entry in fs::read_dir(path).unwrap() {
            let content= fs::read_to_string(entry.unwrap().path()).unwrap();
            let item:Item = ron::from_str(&content).unwrap();
            item_registry.register(item);
        }

        Self {
            base,

            user,
            item_registry
        }
    }

    fn ready(&mut self) {
        self.base_mut()
            .add_to_group("game_manager");
    }
}

impl GameManager {
    pub fn get_player(&self) -> Option<Player>  {
        let tree = Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>();
        let player_node = tree.get_first_node_in_group("player")?;
        let player_ext = player_node.try_cast::<PlayerExt>();

        Some(player_ext.unwrap().bind().player.clone())
    }
}
