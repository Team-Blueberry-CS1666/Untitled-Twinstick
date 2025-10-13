use bevy::{
    prelude::*,
    sprite::Material2dPlugin,
    window::PresentMode,
    winit::cursor::{CursorIcon, CustomCursor, CustomCursorImage},
};

// Game modules
mod components;
mod enemy;
mod events;
mod menu;
mod player;
mod player_material;
mod projectile;
mod tiling;
mod ui;
//mod reticle;
//mod ground_tiles;
//mod ammo_pickup;
//mod guns;
//mod revive_kit_pickup;
mod slideshow;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
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
        // Core game systems
        .add_systems(Startup, setup_cursor_icon)
        //Plugin Section
        .add_plugins((
            menu::MenuPlugin,
            player::PlayerPlugin,
            tiling::TilingPlugin,
            projectile::ProjectilePlugin,
            enemy::EnemyPlugin,
            ui::UIPlugin,
            Material2dPlugin::<player_material::PlayerBaseMaterial>::default(),
            slideshow::CreditsPlugin,
        ))
        .add_event::<events::DamagePlayerEvent>()
        .run();
}