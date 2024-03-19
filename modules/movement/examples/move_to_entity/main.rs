use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use movement_module::{
    move_to_entity::{MoveToEntity, MoveToEntityPlugin},
    move_to_point::MoveToPointPlugin,
    speed::MoveSpeed,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(MoveToEntityPlugin)
        .add_plugins(MoveToPointPlugin)
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, move_tomove)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    /* Add a camera so we can see the debug-render. */
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::ball(20.0))
        .insert(TransformBundle::from(Transform::from_xyz(100.0, 20.0, 0.0)));

    let target = commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(20.0))
        .insert(GravityScale(0.0))
        .insert(ToMove)
        .insert(KinematicCharacterController::default())
        .insert(TransformBundle::from(Transform::from_xyz(400.0, 200.0, 0.0)))
        .id();

    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(30.0))
        .insert(GravityScale(0.0))
        .insert(MoveSpeed(1.0))
        .insert(MoveToEntity(target))
        .insert(KinematicCharacterController::default())
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

fn move_tomove(mut query: Query<&mut KinematicCharacterController, With<ToMove>>) {
    for mut controller in query.iter_mut() {
        controller.translation = Some(Vec2::new(-1.0, 0.));
    }
}

#[derive(Component)]
struct ToMove;
