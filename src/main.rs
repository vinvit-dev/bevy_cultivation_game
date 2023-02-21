use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{Movement, Player, Velocity};
use crate::player::PlayerPlugin;

mod components;
mod player;

pub const RESOLUTION: f32 = 16.0 / 9.0;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 200.;

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
        .add_startup_system(startup_system)
        .add_system(movement_system)
        .run();
}

fn startup_system(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
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

