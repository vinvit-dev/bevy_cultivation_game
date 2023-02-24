use std::fmt::format;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::FixedTimestep;
use crate::components::{Level, Movement, Qi, SpriteSize, Talent, TextChanges, Velocity};
use crate::GameResources;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       app
           .add_startup_system(spawn_player_system)
           .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_status_system)
           .add_system(player_movement_system)
           .add_system(player_levelup_system)
           .add_system_set(
               SystemSet::new()
                   .with_run_criteria(FixedTimestep::step(0.001))
                   .with_system(player_qi_collection_system)
           )
           .add_system(update_player_status_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
)
{
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.7, 0.7, 0.7))),
        transform: Transform {
            scale: Vec3::splat(25.0),
            ..default()
        },
        ..default()
    })
        .insert(Player)
        .insert(Velocity::default())
        .insert(Movement)
        .insert(SpriteSize::from(25.0))
        .insert(Qi::default())
        .insert(Talent::default())
        .insert(Level::default());
}

fn player_movement_system (
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
)
{
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

fn spawn_player_status_system (
    mut commands: Commands,
    game_resorces: Res<GameResources>,
) {
    let text_style = TextStyle {
        font: game_resorces.font_inter_light.clone(),
        font_size: 16.,
        color: Color::WHITE
    };

    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Player status:",
                text_style.clone()
            ),
            TextSection::new(
                "\nLevel: ",
                text_style.clone()
            ),
            TextSection::from_style(text_style.clone()),
            TextSection::new(
                "\nTalent: ",
                text_style.clone()
            ),
            TextSection::from_style(text_style.clone()),
            TextSection::new(
                "\nQi: ",
                text_style.clone()
            ),
            TextSection::from_style(text_style.clone()),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(15.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
    ).insert(TextChanges);

}

fn update_player_status_system (
   mut text_query: Query<&mut Text, With<TextChanges>>,
   player_query: Query<(&Qi, &Talent, &Level), With<Player>>
)
{
    for mut text in text_query.iter_mut() {
       if let Ok((qi, talent, level)) = player_query.get_single() {
           if qi.amount == qi.max_value {
               text.sections[6].value = format!("{:.0} / {:.0}", qi.amount, qi.max_value);
           } else {
               text.sections[6].value = format!("{:.2} / {:.0}", qi.amount, qi.max_value);
           }
           text.sections[4].value = format!("{:.1} / 10", talent.talent);
           text.sections[2].value = format!("{:.0}", level.level);
       }
    }
}

fn player_qi_collection_system (
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Qi, &Velocity), With<Player>>
)
{
    if let Ok((mut qi, velocity)) = query.get_single_mut() {
        if kb.pressed(KeyCode::C) {
            if velocity.x == 0. && velocity.y == 0. {
                if !(qi.amount >= qi.max_value) {
                    qi.amount += qi.speed;
                }
            }
        }
    }
}

fn player_levelup_system (
   kb: Res<Input<KeyCode>>,
   mut query: Query<(&mut Qi, &Velocity, &Talent, &mut Level), With<Player>>
)
{
    if kb.just_pressed(KeyCode::G) {
        if let Ok((mut qi, velocity, talent, mut level))  = query.get_single_mut() {
            if qi.amount >= qi.max_value {
                level.level += 1.;
                qi.amount = 0.;
                qi.speed += qi.speed / (talent.talent * 10.);
                println!("{:}", qi.speed);
                qi.max_value += (qi.max_value / talent.talent).ceil();
            }
        }
    }
}