use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query},
    },
    transform::components::Transform,
};

use crate::move_to_point::MoveToPoint;

pub struct MoveToEntityPlugin;
impl Plugin for MoveToEntityPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_to_entity_system);
    }
}

fn move_to_entity_system(
    mut commands: Commands,
    mut query: Query<(Option<&mut MoveToPoint>, &MoveToEntity, Entity)>,
    trans: Query<&Transform>,
) {
    for (point, &MoveToEntity(target), entity) in query.iter_mut() {
        let Ok(trans) = trans.get(target) else {
            continue;
        };
        if let Some(mut point) = point {
            point.0 = trans.translation.truncate();
        } else {
            commands
                .entity(entity)
                .insert(MoveToPoint(trans.translation.truncate()));
        }
    }
}

#[derive(Component)]
pub struct MoveToEntity(pub Entity);
