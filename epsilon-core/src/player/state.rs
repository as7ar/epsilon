#[derive(Clone)]
pub struct State {
    pub max_hlt: i16,
    pub hlt: i16,
    pub atk: f64,
    pub spd: f32,
}

pub trait IState {
    fn get_max_health(&self) -> i16;
    fn set_max_health(&mut self, new_health: i16);

    fn get_health(&self) -> i16;
    fn set_health(&mut self, new_health: i16);
    fn add_health(&mut self, added_health: i16);

    fn get_attack(&self) -> f64;
    fn set_attack(&mut self, new_attack: f64);
    fn add_attack(&mut self, added_attack: f64);

    fn get_speed(&self) -> f32;
    fn set_speed(&mut self, new_speed: f32);
    fn add_speed(&mut self, added_speed: f32);
}