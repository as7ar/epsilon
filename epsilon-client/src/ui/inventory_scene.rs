use crate::player::PlayerExt;
use godot::classes::{Control, Engine, IControl, Input, SceneTree};
use godot::global::Key;
use godot::obj::{Singleton, WithBaseField};
use godot::prelude::{godot_api, Base, GodotClass};

#[derive(GodotClass)]
#[class(base=Control)]
struct InventoryScene {
    base: Base<Control>,
}

#[godot_api]
impl IControl for InventoryScene {
    fn init(base: Base<Self::Base>) -> Self {

        Self {
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().add_to_group("inventory_ui");
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        if input.is_key_pressed(Key::I) {
            let visible = self.base().is_visible();

            self.base_mut().set_visible(!visible);

            if !visible {
                self.refresh();
            }
        }
    }
}

#[godot_api]
impl InventoryScene {
    #[func]
    pub fn refresh(&mut self) {
        let tree = Engine::singleton()
            .get_main_loop()
            .unwrap()
            .cast::<SceneTree>();

        let player_ext = tree
            .get_first_node_in_group("player")
            .unwrap()
            .cast::<PlayerExt>();
        let player = &player_ext.bind().player;

        for item in player.inventory.items() {
            
        }
    }
}