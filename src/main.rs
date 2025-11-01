use bevy::{
    prelude::*,
    sprite::Material2dPlugin,
    window::PresentMode,
    winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage},
};
use crate::pickup_system::PickupPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

// Game modules
mod camera;
mod client;
mod collectible;
mod components;
mod enemy;
mod events;
mod light_manager;
mod menu;
mod pickup_system;
mod player;
mod player_material;
mod projectile;
mod server;
mod tiling;
mod ui;
mod wall;
//mod reticle;
//mod ground_tiles;
//mod ammo_pickup;
//mod guns;
//mod revive_kit_pickup;
mod collisions;
mod game_over;
mod lobby;
mod slideshow;
mod net_control;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Lobby,
    Joining,
    Credits,
    GameOver,
}

fn setup_cursor_icon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<Entity, With<Window>>,
) {
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image(CustomCursorImage {
            handle: asset_server.load("textures/reticle.png"),
            texture_atlas: None,
            flip_x: false,
            flip_y: false,
            rect: None,
            hotspot: (1, 1),
        })));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Untitled Survival Shooter".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // GameState init
        .init_state::<GameState>()
        .add_plugins(FrameTimeDiagnosticsPlugin{
            max_history_length: 20,
            smoothing_factor: 0.1,
        })
        .add_plugins(LogDiagnosticsPlugin::default())
        //.insert_resource(Time::<Fixed>::from_hz(10.0))
        // Core game systems
        //.add_systems(OnEnter(GameState::Playing), spawn_test_pickup)
        .add_systems(Startup, setup_cursor_icon)
        //Plugin Section
        .add_plugins((
            player::PlayerPlugin,
            light_manager::LightSourcePlugin,
            menu::MenuPlugin,
            tiling::TilingPlugin,
            projectile::ProjectilePlugin,
            enemy::EnemyPlugin,
            collectible::CollectiblePlugin,
            ui::UIPlugin,
        ))
        .add_plugins((
            Material2dPlugin::<player_material::PlayerBaseMaterial>::default(),
            slideshow::CreditsPlugin,
            game_over::GameOverPlugin,
            PickupPlugin,
            camera::CameraPlugin,
            wall::WallPlugin,
            collisions::CollisionsPlugin,
        ))
        .add_plugins((
            lobby::LobbyPlugin,
            server::ServerPlugin,
            client::ClientPlugin,
        ))
        .add_event::<events::DamagePlayerEvent>()
        .run();
}
