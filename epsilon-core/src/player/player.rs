use crate::State;
use crate::User;

#[derive(Clone)]
pub struct Player {
    pub user: User,
    pub state: State
}

impl Player {
    pub fn new(user: User) -> Self {
        Self {
            user,
            state: State {
                max_hlt: 5,
                hlt: 3,
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
}