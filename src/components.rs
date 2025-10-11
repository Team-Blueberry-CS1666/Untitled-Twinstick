use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

impl Health {
    pub fn new(max: i32) -> Self { Self { max, current: max } }
    pub fn damage(&mut self, amount: i32) -> bool { self.current -= amount; self.current <= 0 }
    pub fn heal(&mut self, amount: i32) { self.current += amount; if self.current > self.max { self.current = self.max } }
    pub fn is_dead(&self) -> bool { self.current <= 0 }
}

// Collectibles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectibleKind {
    ReviveKit,
    Ammo,
    Battery,
    Health,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Collectible {
    pub kind: CollectibleKind,
    pub amount: i32,
}

impl Collectible {
    pub fn new(kind: CollectibleKind, amount: i32) -> Self { Self { kind, amount } }
    pub fn revive() -> Self { Self::new(CollectibleKind::ReviveKit, 1) }
    pub fn ammo(amount: i32) -> Self { Self::new(CollectibleKind::Ammo, amount) }
    pub fn battery(amount: i32) -> Self { Self::new(CollectibleKind::Battery, amount) }
    pub fn health(amount: i32) -> Self { Self::new(CollectibleKind::Health, amount) }
}
