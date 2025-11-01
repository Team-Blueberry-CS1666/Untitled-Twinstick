use crate::{GameState, player::Player, player::FireCooldown, net_control::NetControl, net_control::PlayerType};
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

const PROJECTILE_SPEED: f32 = 1000.;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(MouseMemory {last_pos: Vec2::ZERO})
        .add_systems(
            Update,
            projectile_inputs.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            projectile_movement.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Projectile;

#[derive(Resource)]
pub struct MouseMemory{
    pub last_pos: Vec2
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

pub fn projectile_inputs(
    mut commands: Commands,
    mouse_button_io: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<(&Transform, &mut FireCooldown, &NetControl), With<Player>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut pos_history: ResMut<MouseMemory>,
) {
    let not_shooting = !mouse_button_io.pressed(MouseButton::Left);
    
    for (transform, mut cooldown, netcontrol) in player_q
    {
        let projectile_pos = transform.translation;
        let dir = transform.rotation.mul_vec3(Vec3::Y).truncate();

        if !not_shooting && cooldown.tick(time.delta()) && netcontrol.get_type() == PlayerType::Local {
            commands.spawn((
                Sprite::from_image(asset_server.load("textures/bullet.png")),
                Transform::from_scale(Vec3::splat(0.2)).with_translation(projectile_pos),
                Velocity {
                    velocity: dir * PROJECTILE_SPEED,
                },
                Projectile,
            ));
        }
        //THIS IS CURRENTLY BASED ON THE LOCAL PLAYER'S LAST MOUSE POSITION
        else if netcontrol.clicked(MouseButton::Left) && cooldown.tick(time.delta()) && netcontrol.get_type() == PlayerType::Network {
            commands.spawn((
                Sprite::from_image(asset_server.load("textures/bullet.png")),
                Transform::from_scale(Vec3::splat(0.2)).with_translation(projectile_pos),
                Velocity {
                    velocity: dir * PROJECTILE_SPEED,
                },
                Projectile,
            ));
        }
    };

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
