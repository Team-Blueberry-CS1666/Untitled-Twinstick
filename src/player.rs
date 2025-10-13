use crate::{
    components::Health, events::DamagePlayerEvent, player_material::PlayerBaseMaterial, GameState,
};
use bevy::prelude::*;
use bevy::time::Timer;
use bevy::time::TimerMode;
use bevy::window::PrimaryWindow;

const PLAYER_SPEED: f32 = 300.;
const ACCEL_RATE: f32 = 3600.;
const MAX_HEALTH: i32 = 100;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_orientation.run_if(in_state(GameState::Playing)))
            .add_systems(Update, player_damage.run_if(in_state(GameState::Playing)));
    }
}
/*
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), setup_player)
        .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}
*/

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct FireCooldown(Timer);

impl FireCooldown {
    pub fn tick(&mut self, delta: std::time::Duration) -> bool {
        self.0.tick(delta).finished()
    }

    pub fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity {
    velocity: Vec2,
}

impl Velocity {
    fn new() -> Self {
        Self {
            velocity: Vec2::ZERO,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlayerBaseMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(PlayerBaseMaterial {
            color: LinearRgba::BLACK,
            texture: Some(asset_server.load("player/blueberryman.png")),
        })),
        Transform::from_xyz(-300., 0., 10.).with_scale(Vec3::splat(64.)), // Change size of player here: current size: 64. (makes player 64x larger)
                                                                          // you can have a smaller player with 32 and larger player with 128
        Velocity::new(),
        FireCooldown(Timer::from_seconds(0.2, TimerMode::Repeating)),
        Player,
        Health::new(MAX_HEALTH),
    ));
}

pub fn player_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let (mut transform, mut velocity) = player.into_inner();

    let mut dir = Vec2::ZERO;

    if input.pressed(KeyCode::KeyA) {
        dir.x -= 1.;
    }

    if input.pressed(KeyCode::KeyD) {
        dir.x += 1.;
    }

    if input.pressed(KeyCode::KeyW) {
        dir.y += 1.;
    }

    if input.pressed(KeyCode::KeyS) {
        dir.y -= 1.;
    }

    let deltat = time.delta_secs();
    let accel = ACCEL_RATE * deltat;

    **velocity = if dir.length() > 0. {
        (**velocity + (dir.normalize_or_zero() * accel)).clamp_length_max(PLAYER_SPEED)
    } else if velocity.length() > accel {
        **velocity + (velocity.normalize_or_zero() * -accel)
    } else {
        Vec2::ZERO
    };
    let change = **velocity * deltat;

    transform.translation += change.extend(0.);
}

pub fn player_orientation(
    mut players: Query<(&mut MeshMaterial2d<PlayerBaseMaterial>, &Transform), With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<PlayerBaseMaterial>>,
) {
    let Ok(window) = windows.single() else { return; };
    let Ok((camera, camera_transform)) = camera.single() else { return; };
    
    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(cursor_world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            for (mut material, player_transform) in players.iter_mut() {
                let player_position = player_transform.translation.truncate();
                let direction = cursor_world_position - player_position;
                
                if direction.length() > 0.0 {
                    let angle = direction.y.atan2(direction.x);
                    let degrees = angle.to_degrees();
                    
                    // Convert to 0-360 range and rotate so 0° is bottom
                    let normalized_degrees = ((if degrees < 0.0 { degrees + 360.0 } else { degrees }) + 90.0) % 360.0;
                    
                    // Map angle ranges to sprite images
                    // Bottom=0°, Right=90°, Top=180°, Left=270°
                    let sprite_path = if normalized_degrees >= 337.5 || normalized_degrees < 22.5 {
                        "player/blueberryman.png" // 0° (bottom)
                    } else if normalized_degrees >= 22.5 && normalized_degrees < 67.5 {
                        "player/blueberryman45.png" // 45° (bottom right)
                    } else if normalized_degrees >= 67.5 && normalized_degrees < 112.5 {
                        "player/blueberryman90.png" // 90° (right)
                    } else if normalized_degrees >= 112.5 && normalized_degrees < 157.5 {
                        "player/blueberryman135.png" // 135° (top right)
                    } else if normalized_degrees >= 157.5 && normalized_degrees < 202.5 {
                        "player/blueberryman180.png" // 180° (top)
                    } else if normalized_degrees >= 202.5 && normalized_degrees < 247.5 {
                        "player/blueberryman-135.png" // 225° (top left)
                    } else if normalized_degrees >= 247.5 && normalized_degrees < 292.5 {
                        "player/blueberryman-90.png" // 270° (left)
                    } else {
                        "player/blueberryman-45.png" // 315° (bottom left)
                    };
                    
                    // Get the material handle and update its texture
                    if let Some(material_handle) = materials.get_mut(&material.0) {
                        material_handle.texture = Some(asset_server.load(sprite_path));
                    }
                }
            }
        }
    }
}

pub fn player_damage(
    mut events: EventReader<DamagePlayerEvent>,
    mut players: Query<(Entity, &mut Health), With<Player>>,
    mut commands: Commands,
) {
    for damage_event in events.read() {
        for (player, mut player_health) in players.iter_mut() {
            if damage_event.target == player {
                player_health.damage(damage_event.amount);
                if player_health.is_dead() {
                    commands.entity(player).despawn();
                }
            }
        }
    }
}
