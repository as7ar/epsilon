use crate::State;
use crate::User;

pub struct Player {
    user: User,
    state: State
}

impl Player {
    pub fn new(user: User) -> Self {
        Self {
            user,
            state: State {
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