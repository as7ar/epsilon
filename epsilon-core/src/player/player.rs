use crate::{Inventory, State};
use crate::User;

#[derive(Clone)]
pub struct Player {
    pub user: User,
    pub inventory: Inventory,
    pub state: State
}

impl Player {
    pub fn new(user: User) -> Self {
        Self {
            user,
            inventory: Inventory::new(),
            state: State {
                max_hlt: 5,
                hlt: 1,
                atk: 1.,
                spd: 300.
            }
        }
    }
}

pub trait IPlayer {
    fn respawn(&mut self);
    fn game_over(&mut self);

    fn damage(&mut self, damage: i16);

    fn give(&mut self, item: i16);
    fn remove_item(&mut self, item: i16);
}