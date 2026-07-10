pub struct State {
    pub hlt: i16,
    pub atk: f64,
    pub spd: f32,
}

pub trait IState {
    fn get_health(&mut self) -> i16;
    fn set_health(&mut self, new_health: i16);

    fn get_attack(&mut self) -> f64;
    fn set_attack(&mut self, new_attack: f64);
    fn add_attack(&mut self, added_attack: f64);

    fn get_speed(&mut self) -> f64;
    fn set_speed(&mut self, new_attack: f64);
    fn add_speed(&mut self, added_attack: f64);
}