use crate::collectible::{
    Collectible as NewCollectible, CollectibleType as NewCollectibleType, PlayerInventory,
    pickup_flashlight,
};
use crate::components::{
    Collectible as OldCollectible, CollectibleKind as OldCollectibleKind, Health,
};
use crate::net_control::PlayerType;
use crate::net_control::NetControl;
use crate::player_material::PlayerBaseMaterial;
use crate::light_manager::Lights;
use crate::player::Player;
use bevy::prelude::*;

/// how close to pick up
const PICKUP_RADIUS: f32 = 32.0;

/// collecting ammo
#[derive(Event, Debug, Clone, Copy)]
pub struct AmmoPickupEvent {
    pub amount: i32,
}

/// collecting battery
#[derive(Event, Debug, Clone, Copy)]
pub struct BatteryPickupEvent {
    pub amount: i32,
}

/// collecting revive kit
#[derive(Event, Debug, Clone, Copy)]
pub struct ReviveKitPickupEvent;

/// Plugin
pub struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AmmoPickupEvent>()
            .add_event::<BatteryPickupEvent>()
            .add_event::<ReviveKitPickupEvent>()
            .add_systems(Startup, spawn_revive_kit)
            .add_systems(Startup, spawn_battery)
            .add_systems(Update, battery_pickup_system)
            .add_systems(Update, (pickup_system, attach_flashlight_to_player));
    }
}

fn spawn_revive_kit(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("revive kit/Revive Kit_albedo.png")),
        Transform::from_xyz(200.0, 150.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        OldCollectible::revive(),
    ));
    //spawn a second revive kit
    commands.spawn((
        Sprite::from_image(asset_server.load("revive kit/Revive Kit_albedo.png")),
        Transform::from_xyz(-200.0, -150.0, 0.0).with_scale(Vec3::new(0.5, 0.5, 0.5)),
        OldCollectible::revive(),
    ));
}

fn spawn_battery(mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<PlayerBaseMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    lights: Res<Lights>,) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(PlayerBaseMaterial {
            color: LinearRgba::BLUE,
            texture: Some(asset_server.load("textures/battery_albedo.png")),
            lighting: crate::player_material::Lighting {
                ambient_reflection_coefficient: 0.1,
                ambient_light_intensity: 0.1,
                diffuse_reflection_coefficient: 1.0,
                shininess: 40.0,
            },
            lights: lights.lights,
            normal: Some(asset_server.load("textures/battery_normal.png")),
            mesh_rotation: 0.0,
        })),
        Transform::from_xyz(100., 0., 10.).with_scale(Vec3::splat(64.)),
        OldCollectible::battery(500),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(PlayerBaseMaterial {
            color: LinearRgba::BLUE,
            texture: Some(asset_server.load("textures/battery_albedo.png")),
            lighting: crate::player_material::Lighting {
                ambient_reflection_coefficient: 0.1,
                ambient_light_intensity: 0.1,
                diffuse_reflection_coefficient: 1.0,
                shininess: 40.0,
            },
            lights: lights.lights,
            normal: Some(asset_server.load("textures/battery_normal.png")),
            mesh_rotation: 0.0,
        })),
        Transform::from_xyz(400., 0., 10.).with_scale(Vec3::splat(64.)),
        OldCollectible::battery(150),
    ));
}

/// detect collectibles near the player, apply effects, and despawn pickups.
fn pickup_system(
    mut commands: Commands,
    mut ammo_writer: EventWriter<AmmoPickupEvent>,
    mut battery_writer: EventWriter<BatteryPickupEvent>,
    mut revive_writer: EventWriter<ReviveKitPickupEvent>,
    mut player_q: Query<(&Transform, Option<Mut<Health>>, &mut Player), With<Player>>,
    // Old collectibles from components.rs
    old_collectibles_q: Query<(Entity, &Transform, &OldCollectible)>,
    // New collectibles from collectible.rs
    new_collectibles_q: Query<(Entity, &Transform, &NewCollectible)>,
    mut inventory: ResMut<PlayerInventory>,
) {
    // single_mut is the non-deprecated call
    /*let (player_tf, mut player_health_opt, mut player) = match player_q.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };*/

    for (player_tf, mut player_health_opt, mut player) in player_q {

        // Handle old collectible system items (components.rs)
        for (entity, item_tf, col) in old_collectibles_q.iter() {
            if player_tf.translation.distance(item_tf.translation) > PICKUP_RADIUS {
                continue;
            }

            match col.kind {
                OldCollectibleKind::Health => {
                    // borrow the inner Health mutably WITHOUT moving it out of the Option
                    if let Some(h) = player_health_opt.as_deref_mut() {
                        h.heal(col.amount.max(0));
                    }
                }
                OldCollectibleKind::Ammo => {
                    let added = inventory.add_to_reserve(col.amount.max(0));
                    if added > 0 {
                        ammo_writer.write(AmmoPickupEvent { amount: added });
                    }
                }
                OldCollectibleKind::Battery => {
                    battery_writer.write(BatteryPickupEvent {
                        amount: col.amount.max(0),
                    });
                }
                OldCollectibleKind::ReviveKit => {
                    if inventory.revive_kits < inventory.max_revive_kits {
                        inventory.revive_kits += 1;
                        revive_writer.write(ReviveKitPickupEvent);
                        commands.entity(entity).despawn();
                        println!("Collected a revive kit! Total: {}", inventory.revive_kits);

                    }
                }
            }

            //commands.entity(entity).despawn();
        }

        // Handle new collectible system items (collectible.rs)
        for (entity, item_tf, col) in new_collectibles_q.iter() {
            if player_tf.translation.distance(item_tf.translation) > PICKUP_RADIUS {
                continue;
            }

            match col.collectible_type {
                NewCollectibleType::Health(amount) => {
                    if let Some(h) = player_health_opt.as_deref_mut() {
                        h.heal(amount.max(0));
                    }
                }
                NewCollectibleType::Ammo(amount) => {
                    let added = inventory.add_to_reserve(amount.max(0));
                    if added > 0 {
                        ammo_writer.write(AmmoPickupEvent { amount: added });
                    }
                }
                NewCollectibleType::Battery(amount) => {
                    battery_writer.write(BatteryPickupEvent {
                        amount: amount.max(500),
                    });
                }
                NewCollectibleType::ReviveKit => {
                    if inventory.revive_kits < inventory.max_revive_kits {
                        inventory.revive_kits += 1;
                        revive_writer.write(ReviveKitPickupEvent);
                        commands.entity(entity).despawn();
                        println!("Collected a revive kit! Total: {}", inventory.revive_kits);

                    }
                }
                NewCollectibleType::Flashlight => {
                    pickup_flashlight(&mut inventory);
                }
            }

            //commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct FlashlightHeld;

/// Ensure the player's flashlight sprite is attached to the player entity when owned
fn attach_flashlight_to_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    inventory: Res<PlayerInventory>,
    players_q: Query<(Entity, Option<&Children>, &Transform), With<Player>>,
    flashlight_q: Query<Entity, With<FlashlightHeld>>,
) {
    // If no change in inventory, still ensure presence/absence per player
    for (player_entity, children_opt, player_tf) in players_q.iter() {
        let mut has_child_flashlight = false;
        if let Some(children) = children_opt {
            for child in children.iter() {
                if flashlight_q.get(child).is_ok() {
                    has_child_flashlight = true;
                    break;
                }
            }
        }

        if inventory.has_flashlight {
            // Attach if missing
            if !has_child_flashlight {
                // Compensate for parent's scale so flashlight appears at original pixel size
                let sx = if player_tf.scale.x != 0.0 {
                    1.0 / player_tf.scale.x
                } else {
                    1.0
                };
                let sy = if player_tf.scale.y != 0.0 {
                    1.0 / player_tf.scale.y
                } else {
                    1.0
                };
                // Also compensate the local offset so it stays ~20px to the right visually
                let offset_x = 40.0 * sx; // 20px to the right of the player, increase to push it further right, decrease to push it further left
                commands.entity(player_entity).with_children(|cb| {
                    cb.spawn((
                        Sprite::from_image(asset_server.load("textures/flashlight.png")),
                        // Position slightly to the right of the player, compensate for parent scale
                        Transform::from_xyz(offset_x, 0.0, 1.0).with_scale(Vec3::new(sx, sy, 1.0)),
                        FlashlightHeld,
                    ));
                });
            }
        } else {
            // Remove if present
            if let Some(children) = children_opt {
                for child in children.iter() {
                    if flashlight_q.get(child).is_ok() {
                        commands.entity(child).despawn();
                    }
                }
            }
        }
    }
}

// This code manages picking up a battery!! It's run whenever the signal
// that a battery has been collected is sent.
// Max battery charge is 500, for now.
fn battery_pickup_system(
    mut events: EventReader<BatteryPickupEvent>,
    mut players: Query<(&mut Player, &NetControl)>,
) {
    for event in events.read() {
        for (mut player, net_control) in &mut players {
            if net_control.player_type == PlayerType::Local {
                player.charge_battery(event.amount);
            }
        }
    }
}