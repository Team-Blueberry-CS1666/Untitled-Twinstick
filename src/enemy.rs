use bevy::prelude::*;
use crate::{GameState,
            player::Player,
            projectile::Projectile,
            components::Health,
            events::DamagePlayerEvent,};
use std::f32::consts;

// Stats for different enemy types!
const NORMAL_SPEED: f32 = 300.;
const STRONG_SPEED: f32 = 100.;
const FAST_SPEED: f32 = 600.;

const NORMAL_HEALTH: i32 = 100;
const STRONG_HEALTH: i32 = 500;
const FAST_HEALTH: i32 = 50;

const RADIUS: f32 = 50.;
const ATTACK_RADIUS: f32 = 100.;

const ACCEL_RATE: f32 = 10000.;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), setup_enemy)
        .add_systems(Update, enemy_movement.run_if(in_state(GameState::Playing)))
        .add_systems(Update, enemy_damage.run_if(in_state(GameState::Playing)))
        .add_systems(Update, all_enemies_defeated.run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::GameOver), display_game_over);
        //.add_systems(Update, enemy_attack.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Enemy {
    enemy_type: EnemyType,
    enemy_speed: f32
}

impl Enemy {
    fn new(enemy_type: EnemyType) -> Enemy {
        let enemy_speed = match enemy_type {
            EnemyType::Normal => NORMAL_SPEED,
            EnemyType::Strong => STRONG_SPEED,
            EnemyType::Fast => FAST_SPEED,
        };
        Enemy {enemy_type, enemy_speed}
    }
}

enum EnemyType {
    Normal,
    Strong,
    Fast
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

pub fn setup_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..=9 {
        commands.spawn((
            Sprite::from_image(asset_server.load("enemy/enemy_standard_albedo.png")),
            Transform::from_xyz(300., (i * 100) as f32, 10.),
            Velocity::new(),
            Enemy::new(EnemyType::Strong),
            Health::new(STRONG_HEALTH),
        ));
    }
}

pub fn enemy_movement(
    time: Res<Time>,
    mut params: ParamSet<(
        Query<(&Enemy, &mut Transform, &mut Velocity), With<Enemy>>,
        Single<&Transform, With<Player>>,
    )>,
) {
    let player_transform = params.p1().into_inner().clone();
    for (enemy, mut enemy_transform, mut velocity) in params.p0().iter_mut() {
        // Create a vector FROM the enemy TO the player target.
        let mut dir = Vec2::ZERO;
        dir.x = player_transform.translation.x - enemy_transform.translation.x;
        dir.y = player_transform.translation.y - enemy_transform.translation.y;

        let deltat = time.delta_secs();
        let accel = ACCEL_RATE * deltat;

        **velocity = if dir.length() > 0. {
            (**velocity + (dir.normalize_or_zero() * accel)).clamp_length_max(enemy.enemy_speed)
        } else if velocity.length() > accel {
            **velocity + (velocity.normalize_or_zero() * -accel)
        } else {
            Vec2::ZERO
        };
        let change = **velocity * deltat;

        enemy_transform.translation += change.extend(0.);

        let rotation_z = dir.y.atan2(dir.x);
        enemy_transform.rotation = Quat::from_rotation_z(rotation_z - consts::PI / 2.);
    }
} 

pub fn enemy_attack(
    enemies: Query<&Transform, With<Enemy>>,
    player: Single<(Entity, &Transform), With<Player>>,
    mut event: EventWriter<DamagePlayerEvent>,
) {
    let (player_entity, player_transform) = player.into_inner();
    for enemy_transform in enemies.iter() {
        let distance = (enemy_transform.translation - player_transform.translation).length();
        if distance < ATTACK_RADIUS {
            event.write(DamagePlayerEvent::new(player_entity, 1));
        }
    }
}

pub fn enemy_damage(
    mut enemies: Query<(Entity, &Transform, &mut Health), With<Enemy>>,
    projectiles: Query<&Transform, With<Projectile>>,
    mut commands: Commands
) {
    for (enemy, enemy_transform, mut enemy_health) in enemies.iter_mut() {
        for projectile_transform in projectiles.iter() {
            let distance = (enemy_transform.translation - projectile_transform.translation).length();
            if distance > RADIUS {
                continue;
            }
            // Damage, then check if enemy is dead...
            if enemy_health.damage(10) {
                commands.entity(enemy).despawn();
            }
        }
    }
} 

pub fn all_enemies_defeated(
    all_enemies: Query<&Health, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let mut all_enemies_dead = true;
    for enemy in all_enemies.iter() {
        if enemy.is_dead() == false {
            all_enemies_dead = false;
            break;
        }
    }
    if all_enemies_dead {
        next_state.set(GameState::GameOver);
    }
}

pub fn display_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            //MenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 96.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)), //red
            ));
        });
}