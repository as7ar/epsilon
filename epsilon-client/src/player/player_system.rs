use crate::player::player_scene::PlayerExt;
use epsilon_core::IState;

impl IState for PlayerExt {
    fn get_max_health(&self) -> i16 {
        self.player.state.max_hlt
    }

    fn set_max_health(&mut self, new_health: i16) {
        self.player.state.max_hlt= new_health
    }
    
    fn get_health(&self) -> i16 {
        self.player.state.hlt
    }

    fn set_health(&mut self, new_health: i16) {
        self.player.state.hlt=new_health
    }

    fn add_health(&mut self, added_health: i16) {
        self.player.state.hlt += added_health;
    }

    fn get_attack(&self) -> f64 {
        self.player.state.atk
    }

    fn set_attack(&mut self, new_attack: f64) {
        self.player.state.atk=new_attack
    }

    fn add_attack(&mut self, added_attack: f64) {
        self.player.state.atk+=added_attack
    }

    fn get_speed(&self) -> f32 {
        self.player.state.spd
    }

    fn set_speed(&mut self, new_speed: f32) {
        self.player.state.spd=new_speed
    }

    fn add_speed(&mut self, added_speed: f32) {
        self.player.state.spd+=added_speed
    }
}