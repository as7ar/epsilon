use crate::player::PlayerExt;
use godot::classes::{Engine, HBoxContainer, IHBoxContainer, ResourceLoader, TextureRect};
use godot::obj::{Base, Singleton, WithBaseField, WithUserSignals};
use godot::prelude::{godot_api, GodotClass, PackedScene, SceneTree, ToGodot, Vector2};

#[derive(GodotClass)]
#[class(base=HBoxContainer)]
struct HealthBar {
    base: Base<HBoxContainer>,

    pub(crate) max_health: i16,
}

#[godot_api]
impl IHBoxContainer for HealthBar {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            max_health: 0,
        }
    }

    fn ready(&mut self) {
        let tree = Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>();

        let player_ext = tree
            .get_first_node_in_group("player")
            .unwrap()
            .cast::<PlayerExt>();
        let player = &player_ext.bind().player;

        let mut health = player.state.hlt;
        self.max_health = player.state.max_hlt;

        let mut this = self.base().to_godot_owned();
        player_ext.signals()
            .health_changed()
            .connect_self(move |_, changed| {
                health = changed;

                // godot_print!("{}", health);

                this.call("update_health", &[health.to_variant()]);
            });

        self.create_health_bar(health);
    }
}

#[godot_api]
impl HealthBar {
    #[func]
    fn create_health_bar(&mut self, health: i16) {
        let loader = ResourceLoader::singleton();

        let heart = loader.clone()
            .load("res://scene/ui/heart.tscn")
            .unwrap()
            .cast::<PackedScene>();
        let empty_heart = loader.clone()
            .load("res://scene/ui/empty_heart.tscn")
            .unwrap()
            .cast::<PackedScene>();

        let max_health = self.max_health;

        for i in 0..max_health {
            let mut heart = if i < health {
                heart.instantiate().unwrap().cast::<TextureRect>()
            } else {
                empty_heart.instantiate().unwrap().cast::<TextureRect>()
            };

            heart.set_custom_minimum_size(Vector2::new(32.0, 32.0));
            self.base_mut().add_child(&heart);
        }
    }

    #[func]
    fn update_health(&mut self, health: i16) {
        let children = self.base_mut().get_children();
        for mut child in children.iter_shared() {
            child.queue_free();
        }

        // godot_print!("{}", health);
        self.create_health_bar(health);
    }
}