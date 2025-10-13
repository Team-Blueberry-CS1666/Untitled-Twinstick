use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {max, current: max}
    }
    pub fn damage(&mut self, amount: i32) -> bool {
        self.current -= amount;
        self.current <= 0
    }
    pub fn heal(&mut self, amount: i32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max
        }
    }
    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }
}