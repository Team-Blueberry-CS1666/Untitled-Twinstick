use bevy::ecs::entity::unique_slice::Windows;
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::TextureCache;
use bevy::render::{
    render_graph::*,
    render_resource::*,
    renderer::RenderContext,
    ExtractSchedule, RenderApp, Render, RenderSet,
};
use bevy::window::{self, PrimaryWindow, WindowResolution};
use bevy::core_pipeline::core_2d::graph::Core2d;

pub struct DeferredLitePlugin;

impl Plugin for DeferredLitePlugin {
    fn build(&self, app: &mut App) {
        // Getting the reference to the RenderApp
        let render_app = app.sub_app_mut(RenderApp);

        // TODO: will have to figure out how to add the system for the prepare stage of the render pass wihtout using RenderSet
        // render_app.add_systems(Startup, prepare_normal_render_target.in_set(RenderSet::Prepare));
    }
}

#[derive(Resource)]
struct GBufferLite {
    texture_view: Option<TextureView>,
    size: UVec2,
    format: TextureFormat
}

// gBufRes -> our resource that will hold everything for out render targets such as normals
// textures -> Bevy's render
// This is for the prepare stage of the render world for Bevy rendering
fn prepare_normal_render_target(
    mut gbufRes: ResMut<GBufferLite>,
    mut textures: ResMut<TextureCache>,
    window: Query<&Window, With<PrimaryWindow>>,
    render_dev: Res<RenderDevice> 
) {
    // Update the size of the window if needed
    let primary_window = match window.single() {
        Ok(v) => v,
        Err(_) => return
    };

    // TODO: maybe we can skip
    // if primary_window.resolution.size() == gbufRes.size {return;}
    gbufRes.size = primary_window.resolution.physical_size();
    gbufRes.format = TextureFormat::Rgba8Unorm;

    let texture_desc = TextureDescriptor {
        label: Some("deferred_lite_normals"),
        size: Extent3d { width: gbufRes.size.x, height: gbufRes.size.y, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: gbufRes.format,
        usage: TextureUsages::COPY_SRC | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    };

    let cached = textures.get(&render_dev, texture_desc);
    gbufRes.texture_view = Some(cached.default_view);

    println!("{}", primary_window.width());
}

// TODO: need to create a NODE for the render stage of the main pass to handle the normals