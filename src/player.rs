use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::components::{Movement, Player, Velocity};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_startup_system(spawn_player_system)
           .add_system(player_movement_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.7))),
        transform: Transform {
            scale: Vec3::splat(25.),
            ..default()
        },
        ..default()
    })
        .insert(Player)
        .insert(Velocity::default())
        .insert(Movement);
}

fn player_movement_system (
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::A) {
            -1.
        } else if kb.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };

        velocity.y = if kb.pressed(KeyCode::W) {
            1.
        } else if kb.pressed(KeyCode::S) {
            -1.
        } else {
            0.
        }
    }
}