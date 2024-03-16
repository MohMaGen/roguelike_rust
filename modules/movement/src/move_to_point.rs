use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query},
    },
    math::Vec2,
    transform::components::Transform,
};
use bevy_rapier2d::{control::KinematicCharacterController, dynamics::Velocity};

use crate::speed::MoveSpeed;

pub struct MoveToPointPlugin;
impl Plugin for MoveToPointPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, move_to_point_system);
    }
}

/// Change velocity of entity with *MoveToPoint*, to make it moving to point.
///
/// Entity will move with speed of *MoveSpeed* component.
pub fn move_to_point_system(
    mut commands: Commands,
    mut query: Query<(&mut KinematicCharacterController, &MoveToPoint, &MoveSpeed, &Transform, Entity)>,
) {
    for (mut controller, &MoveToPoint(point), &MoveSpeed(speed), trans, ent) in query.iter_mut() {
        let diff = point - trans.translation.truncate();

        if diff.length() <= MAX_ACHIVE_POINT_ERROR {
            commands.entity(ent).remove::<MoveToPoint>();
            controller.translation = None;
        } else {
            controller.translation = Some(diff.normalize_or_zero() * speed);
        }
    }
}

/// Component defines point to which move entity.
#[derive(Component, Default)]
pub struct MoveToPoint(pub Vec2);

const MAX_ACHIVE_POINT_ERROR: f32 = 1e-2;
