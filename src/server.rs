// src/server.rs
use bevy::prelude::*;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, _app: &mut App) {
        // No-op for now. Real networking logic can be added later.
    }
}
