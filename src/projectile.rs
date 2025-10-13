use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use crate::{GameState, player:: Player, player::FireCooldown};

const PROJECTILE_SPEED: f32 = 1000.;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, projectile_inputs.run_if(in_state(GameState::Playing)))
        .add_systems(Update, projectile_movement.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Projectile;

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

pub fn projectile_inputs(
    mut commands: Commands,
    mouse_button_io: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<(&Transform, &mut FireCooldown), With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let not_shooting = !mouse_button_io.pressed(MouseButton::Left);

    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let (camera, camera_transform) = match camera_q.single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let Some(cursor_screen_pos) = window.cursor_position() else {
        return;
    };
    let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos)
    else {
        return;
    };
    let (transform, mut cooldown) = match player_q.single_mut() {
        Ok(v) => v,
        Err(_) => return, // no or multiple players; bail out safely
    };

    let projectile_pos = transform.translation;
    let dir = (cursor_world_pos - projectile_pos.truncate()).normalize();

    if !not_shooting && cooldown.tick(time.delta()) {
        commands.spawn((
        Sprite::from_image(asset_server.load("textures/bullet.png")),   
        Transform::from_scale(Vec3::splat(0.2)).with_translation(projectile_pos),
        Velocity {
            velocity: dir * PROJECTILE_SPEED,
        },
        Projectile,
    ));
    }
}

pub fn projectile_movement(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &Velocity), With<Projectile>>,
) {
    for (mut transform, velocity) in &mut projectiles {
        let delta_t = time.delta_secs();
        let delta_d = **velocity * delta_t;
        transform.translation += delta_d.extend(0.);
    }
}