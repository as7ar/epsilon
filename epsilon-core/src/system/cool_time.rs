use std::time::Instant;

pub struct CoolDown {
    can_use: bool,
    cooldown_ms: f64,
    last_used: Option<Instant>,
}

impl CoolDown {
    pub fn new(cooldown_ms: f64) -> Self {
        Self {
            can_use: true,
            cooldown_ms,
            last_used: None,
        }
    }

    pub fn try_use(&mut self) -> bool {
        if !self.can_use {
            return false;
        }

        self.can_use = false;
        self.last_used = Some(Instant::now());
        true
    }

    pub fn tick(&mut self) {
        if self.can_use {
            return;
        }

        let Some(last) = self.last_used else {
            self.can_use = true;
            return;
        };

        let elapsed = last.elapsed().as_secs_f64() * 1000.0;

        if elapsed >= self.cooldown_ms {
            self.can_use = true;
            self.last_used = None;
        }
    }

    pub fn ready(&self) -> bool {
        self.can_use
    }
}