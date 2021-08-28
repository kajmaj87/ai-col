use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Theme<'a> {
    ground: &'a String,
}

#[derive(Deserialize)]
struct Config {
    scale: f32,
}

fn config_load(mut commands: Commands) {
    let config: Config = toml::from_str(
        &fs::read_to_string("config.toml")
            .expect("Something went wrong reading the config file!"),
    )
    .expect("Something went wrong parsing the config file!");
    let theme: Theme = toml::from_str(
        &fs::read_to_string("themes/tiles.toml")
            .expect("Something went wrong reading the theme file!"),
    )
    .expect("Something went wrong parsing the theme file!");
    commands.insert_resource(theme);
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, config: Res<Config>, theme: Res<Theme>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::hex(theme.ground).unwrap().into()),
        sprite: Sprite::new(Vec2::new(
            config.scale,
            config.scale,
        )),
        transform: Transform::default(),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}
