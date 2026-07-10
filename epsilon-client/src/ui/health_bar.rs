use godot::classes::{Engine, HBoxContainer, IHBoxContainer, ResourceLoader, TextureRect};
use godot::global::godot_print;
use godot::obj::{Base, NewAlloc, Singleton, WithBaseField};
use godot::prelude::{godot_api, GodotClass, PackedScene, SceneTree, Vector2};
use crate::game_manager::GameManager;

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
            let mut heart = scene.instantiate()
                .unwrap().cast::<TextureRect>();
            heart.set_custom_minimum_size(Vector2 {
                x: 48., y: 48.
            });

            self.base_mut().add_child(&heart);
        }
    }
}