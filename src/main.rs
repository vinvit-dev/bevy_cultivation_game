use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin};
use crate::components::{Movement, Player, Velocity};
use crate::player::PlayerPlugin;

mod components;
mod player;

pub const RESOLUTION: f32 = 16.0 / 9.0;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 200.;

#[derive(Resource)]
pub struct GameResources {
    InterLightFont: Handle<Font>,
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
                ..default()
            },
            ..default()
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(startup_system)
        .add_system(movement_system)
        .run();
}

fn startup_system(
    mut commands: Commands,
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
    assets_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }

    let game_resources = GameResources {
        InterLightFont: assets_server.load("fonts/Inter-Light.ttf"),
    };


    commands.spawn(
        TextBundle::from_section(
            "Test text",
            TextStyle {
                font: game_resources.InterLightFont.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            }
        ).with_style(
            Style {
               position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }
        )
    );
    commands.insert_resource(game_resources);
}


fn movement_system (
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Movement>>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
       let transtaltion = &mut transform.translation;
        transtaltion.x += velocity.x * TIME_STEP * BASE_SPEED;
        transtaltion.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

