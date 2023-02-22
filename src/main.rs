use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::global_const::RESOLUTION;
use crate::global_systems::GlobalSystemsPlugin;
use crate::player::PlayerPlugin;

mod components;
mod player;
mod global_systems;
mod global_const;


#[derive(Resource)]
pub struct GameResources {
    font_inter_light: Handle<Font>,
}

fn main() {
    let height: f32 = 540.0;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: height * RESOLUTION,
                height,
                title: "Cool game".to_string(),
                resizable: true,
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(GlobalSystemsPlugin)
        .add_startup_system(startup_system)
        .run();
}

fn startup_system(
    mut commands: Commands,
    assets_server: Res<AssetServer>
)
{
    commands.spawn(Camera2dBundle::default());


    let game_resources = GameResources {
        font_inter_light: assets_server.load("fonts/Inter-Light.ttf"),
    };

    commands.insert_resource(game_resources);
}


