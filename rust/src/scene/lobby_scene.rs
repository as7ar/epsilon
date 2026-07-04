use godot::obj::Base;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct LobbyScene {
    base: Base<Node2D>,
    started: bool,
}

#[godot_api]
impl INode2D for LobbyScene {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            started: false
        }
    }

    fn ready(&mut self) {

    }
}