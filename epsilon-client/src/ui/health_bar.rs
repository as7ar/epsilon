use crate::game_manager::GameManager;
use godot::classes::{Engine, HBoxContainer, IHBoxContainer, ResourceLoader, TextureRect};
use godot::global::godot_print;
use godot::obj::{Base, Singleton, WithBaseField, WithUserSignals};
use godot::prelude::{godot_api, GodotClass, PackedScene, SceneTree, ToGodot, Vector2};
use crate::player::PlayerExt;

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
struct HealthBar {
    base: Base<HBoxContainer>
}

#[godot_api]
impl IHBoxContainer for HealthBar {
    fn init(base: Base<Self::Base>) -> Self {
        /*let engine = &Engine::singleton()
            .get_singleton("GameManager")
            .unwrap().cast::<GameManager>().bind().player;*/

        Self {
            base
        }
    }

    fn ready(&mut self) {
        let tree = Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>();

        let manager = tree
            .get_first_node_in_group("game_manager")
            .unwrap()
            .cast::<GameManager>();

        let player_ext = tree
            .get_first_node_in_group("player")
            .unwrap()
            .cast::<PlayerExt>();

        let mut this = self.base().to_godot_owned();
        player_ext.signals()
            .damage_taken()
            .connect_self(move |_, amount| {
                this.call("update_health", &[amount.to_variant()]);
            });

        let player = &manager.bind().player;
        let health = player.state.hlt;

        self.create_health_bar(health);
    }
}

#[godot_api]
impl HealthBar {
    #[func]
    fn create_health_bar(&mut self, health: i16) {
        let scene = ResourceLoader::singleton()
            .load("res://scene/ui/heart.tscn")
            .unwrap()
            .cast::<PackedScene>();

        for _ in 1..=health {
            let mut heart = scene.instantiate().unwrap().cast::<TextureRect>();
            heart.set_custom_minimum_size(Vector2 {
                x: 32., y: 32.
            });

            self.base_mut().add_child(&heart);
        }
    }

    #[func]
    fn update_health(&mut self, health: i16) { // 호출 안 됨
        godot_print!("{health}");
        self.base_mut().get_children().clear();
        self.create_health_bar(health);
    }
}