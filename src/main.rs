use bevy::{diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use serde::Deserialize;
use std::fs;

use diagnostic_plugin::DiagnosticPlugin;

mod diagnostic_plugin;
mod helpers;

#[derive(Deserialize)]
struct Config {
    map_width: u32,
    map_height: u32,
    tile_scale: f32,
    camera_zoom_speed: f32,
    camera_max_zoom: f32,
    camera_min_zoom: f32,
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Ground,
    Resource,
}

fn config_load(mut commands: Commands) {
    let config: Config = toml::from_str(
        &fs::read_to_string("config.toml").expect("Something went wrong reading the config file!"),
    )
    .expect("Something went wrong parsing the config file!");
    commands.insert_resource(config);
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
    config:Res<Config>
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 1.0 / config.tile_scale;
    commands.spawn_bundle(camera);

    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(config.map_width, config.map_height),
            ChunkSize(16, 16),
            TileSize(16.0, 16.0),
            TextureSize(96.0, 16.0),
        ),
        0u16,
        0u16,
        None,
    );
    layer_builder.set_all(TileBundle::default());

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}

fn update_map(mut map_query: MapQuery, mut commands: Commands){
        let position = TilePos(2, 5);
        // Ignore errors for demo sake.
        let _ = map_query.set_tile(
            &mut commands,
            position,
            Tile {
                texture_index: 2,
                ..Default::default()
            },
            0u16,
            0u16,
        );
        map_query.notify_chunk_for_tile(position, 0u16, 0u16);
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("AI-Col"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(DiagnosticPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, config_load.system())
        .add_startup_system(startup.system())
        .add_system(update_map.system())
        .add_system(helpers::camera::movement.system())
        .add_system(helpers::texture::set_texture_filters_to_nearest.system())
        .run();
}
