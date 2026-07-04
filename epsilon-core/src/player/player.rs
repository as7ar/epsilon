use crate::State;
use crate::User;

pub struct Player {
    user: User,
    state: State
}

pub trait IPlayer {}