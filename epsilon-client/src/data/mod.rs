pub mod player_state;

use directories::ProjectDirs;
use epsilon_core::User;
pub use player_state::*;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn init() -> PathBuf {
    let path = ProjectDirs::from("com", "catowerstudio", "epsilon")
        .unwrap().data_dir().to_path_buf();

    let user = get_user();
    println!("user_id: {}\ncreated_at: {}", user.id.to_string(), user.created_at);

    path
}

pub fn get_user() -> User {
    let path = init();
    let client = path.join("client");
    let user: User;
    if !client.exists() {
        let uuid = Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH).expect("Time went backwards")
            .as_secs();

        user = User {
            id: uuid,
            nickname: "".to_string(),
            created_at: timestamp
        };

        let ron = ron::to_string(&user).unwrap();

        fs::write(&path, ron).unwrap();
    } else {
        let text = fs::read_to_string(&path).unwrap();
        user = ron::from_str(&text).unwrap()
    }
    user
}