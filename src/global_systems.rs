use bevy::prelude::*;
use bevy_egui::EguiSettings;
use crate::components::{Movement, Velocity};
use crate::global_const::{BASE_SPEED, TIME_STEP};

pub struct GlobalSystemsPlugin;

impl Plugin for GlobalSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(startup_system)
            .add_system(movement_system);
    }
}

fn movement_system (
    mut query: Query<(Entity, &Velocity, &mut Transform), With<Movement>>,
)
{
    for (_entity, velocity, mut transform) in query.iter_mut() {
        let transtaltion = &mut transform.translation;
        transtaltion.x += velocity.x * TIME_STEP * BASE_SPEED;
        transtaltion.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

fn startup_system(
    mut egui_settings: ResMut<EguiSettings>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        egui_settings.scale_factor = 1.0 / window.scale_factor();
    }
}
