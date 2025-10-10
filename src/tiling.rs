use bevy::{prelude::*};
use crate::{GameState, WIN_H, WIN_W};

const TILE_SIZE: u32 = 128;

pub struct TilingPlugin;
impl Plugin for TilingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_tiling);
    }
}

#[derive(Component)]
struct Tile;

pub fn setup_tiling(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let ground_handle = asset_server.load("textures/ground.png");
    let ground_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);
    let ground_layout_len = ground_layout.textures.len();
    let ground_layout_handle = texture_atlases.add(ground_layout);

    //We currently only tile over the 1280x720 window, we can adjust this to instead work with level size
    let x_bound = WIN_W / 2. - (TILE_SIZE as f32) / 2.;
    let y_bound = WIN_H / 2. - (TILE_SIZE as f32) / 2.;

    let mut x = 0;
    let mut y = 0;
    let mut t = Vec3::new(-x_bound, -y_bound, -10.);
    while (y as f32) * (TILE_SIZE as f32) < WIN_H {
        while (x as f32) * (TILE_SIZE as f32) < WIN_W {
            commands.spawn((
                Sprite::from_atlas_image(
                    ground_handle.clone(),
                    TextureAtlas {
                        layout: ground_layout_handle.clone(),

                        //This will randomly select a tile from the map, to add variety
                        index: rand::random_range(0..ground_layout_len),
                    },
                ),
                Transform {
                    translation: t,
                    ..default()
                },
                Tile,
            ));

            x += 1;
            t += Vec3::new(TILE_SIZE as f32, 0., 0.);
        }
        x = 0;
        t = Vec3::new(-x_bound, -y_bound, -10.);

        y += 1;
        t += Vec3::new(0., (y as f32) * (TILE_SIZE as f32), 0.);
    }
}