use crate::data::get_user;
use godot::classes::{ILabel, Label};
use godot::obj::{Base, WithBaseField};
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Label)]
pub(crate) struct InfoLabel {
    base: Base<Label>
}

#[godot_api]
impl ILabel for InfoLabel {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self) {
        let user = get_user();
        let uuid = user.id.to_string();
        let version = env!("CARGO_PKG_VERSION");

        self.base_mut().set_text(&format!("uuid: {uuid}      ver{version}"))
    }
}