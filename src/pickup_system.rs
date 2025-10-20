use crate::collectible::{
    Collectible as NewCollectible, CollectibleType as NewCollectibleType, PlayerInventory,
    pickup_flashlight,
};
use crate::components::{
    Collectible as OldCollectible, CollectibleKind as OldCollectibleKind, Health,
};
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

/// detect collectibles near the player, apply effects, and despawn pickups.
fn pickup_system(
    mut commands: Commands,
    mut ammo_writer: EventWriter<AmmoPickupEvent>,
    mut battery_writer: EventWriter<BatteryPickupEvent>,
    mut revive_writer: EventWriter<ReviveKitPickupEvent>,
    mut player_q: Query<(&Transform, Option<Mut<Health>>), With<Player>>,
    // Old collectibles from components.rs
    old_collectibles_q: Query<(Entity, &Transform, &OldCollectible)>,
    // New collectibles from collectible.rs
    new_collectibles_q: Query<(Entity, &Transform, &NewCollectible)>,
    mut inventory: ResMut<PlayerInventory>,
) {
    // single_mut is the non-deprecated call
    let (player_tf, mut player_health_opt) = match player_q.single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

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
                ammo_writer.write(AmmoPickupEvent {
                    amount: col.amount.max(0),
                });
            }
            OldCollectibleKind::Battery => {
                battery_writer.write(BatteryPickupEvent {
                    amount: col.amount.max(0),
                });
            }
            OldCollectibleKind::ReviveKit => {
                if let Some(h) = player_health_opt.as_deref_mut() {
                    h.current = h.max; //refill health
                }

                revive_writer.write(ReviveKitPickupEvent);
            }
        }

        commands.entity(entity).despawn();
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
                ammo_writer.write(AmmoPickupEvent {
                    amount: amount.max(0),
                });
            }
            NewCollectibleType::Battery(amount) => {
                battery_writer.write(BatteryPickupEvent {
                    amount: amount.max(0),
                });
            }
            NewCollectibleType::ReviveKit => {
                if let Some(h) = player_health_opt.as_deref_mut() {
                    h.current = h.max; // refill health
                }
                revive_writer.write(ReviveKitPickupEvent);
            }
            NewCollectibleType::Flashlight => {
                pickup_flashlight(&mut inventory);
            }
        }

        commands.entity(entity).despawn();
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
