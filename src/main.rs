use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

use diagnostic_plugin::DiagnosticPlugin;

mod diagnostic_plugin;

#[derive(Deserialize)]
struct Theme {
    ground: String,
}

#[derive(Deserialize)]
struct Config {
    tile_scale: f32,
    tile_border_size: f32,
    map_width: usize,
    map_height: usize
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Ground,
    Resource,
}

struct Map {
    tiles: Vec<TileType>,
    width: usize,
    height: usize,
}

impl Map {
    fn get_x(&self, i: usize) -> usize{
        i % self.width
    } 
    fn get_y(&self, i: usize) -> usize{
        i / self.width
    } 
}

fn new_map(width: usize, height: usize) -> Map {
    let tiles = vec![TileType::Ground; width * height];
    Map {
        tiles,
        width,
        height,
    }
}

fn config_load(mut commands: Commands) {
    let config: Config = toml::from_str(
        &fs::read_to_string("config.toml").expect("Something went wrong reading the config file!"),
    )
    .expect("Something went wrong parsing the config file!");
    let theme: Theme = toml::from_str(
        &fs::read_to_string("themes/tiles.toml")
            .expect("Something went wrong reading the theme file!"),
    )
    .expect("Something went wrong parsing the theme file!");
    commands.insert_resource(new_map(config.map_width, config.map_height));
    commands.insert_resource(config);
    commands.insert_resource(theme);
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>,
    config: Res<Config>,
    theme: Res<Theme>,
    time: Res<Time>
) {
    println!("Starting resource creation {:?}", time.delta_seconds());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    let sprite = Sprite::new(Vec2::new(
                config.tile_scale - config.tile_border_size,
                config.tile_scale - config.tile_border_size,
            ));

    for (i, tile) in map.tiles.iter().enumerate() {
        commands.spawn_bundle(SpriteBundle {
            material: materials.add(Color::hex(theme.ground.clone()).unwrap().into()),
            sprite: sprite.clone(),
            transform: Transform::from_xyz(
                config.tile_scale * map.get_x(i) as f32 - map.width as f32 * config.tile_scale / 2.0,
                config.tile_scale * map.get_y(i) as f32 - map.height as f32 * config.tile_scale / 2.0,
                0.0,
            ),
            ..Default::default()
        });
    }
    println!("Ending resource creation {:?}", time.delta_seconds());
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DiagnosticPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, config_load.system())
        .add_startup_system(setup.system())
        .run();
}
