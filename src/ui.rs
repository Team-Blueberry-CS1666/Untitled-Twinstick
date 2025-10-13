use bevy::prelude::*;
use crate::{GameState,
            components::Health,
            events::DamagePlayerEvent,
            player::Player};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), setup_ui)
        .add_systems(Update, player_damage.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct HealthBar;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // This is the background for health UI!
    commands.spawn((
        Sprite::from_image(asset_server.load("textures/rust.png")),
        Transform::from_xyz(-576., 200., 999.).with_scale(Vec3::new(0.25, 1., 1.)),
    ));
    // This is the actual health meter.
    commands.spawn((
        Sprite::from_image(asset_server.load("textures/health.png")),
        Transform::from_xyz(-584., 192., 999.).with_scale(Vec3::new(1., 1., 1.)),
        HealthBar,
    ));

}

pub fn player_damage(
    mut events: EventReader<DamagePlayerEvent>,
    health_bar: Single<&mut Transform, With<HealthBar>>,
    players: Query<(Entity, &Health), With<Player>>,
) {
    let mut transform = health_bar.into_inner();
    for damage_event in events.read() {
        for (player, player_health) in players.iter() {
            if damage_event.target == player {
                let y_scale = player_health.current as f32 / player_health.max as f32;
                transform.scale = Vec3::new(1., y_scale, 1.);
            }
        }
    }
}