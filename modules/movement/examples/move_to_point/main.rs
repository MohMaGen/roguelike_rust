use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use movement_module::{
    move_to_point::{MoveToPoint, MoveToPointPlugin},
    speed::MoveSpeed,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(MoveToPointPlugin)
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, log)
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

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(30.0))
        .insert(GravityScale(0.0))
        .insert(MoveSpeed(1.0))
        .insert(MoveToPoint(Vec2::new(400., 0.)))
        .insert(KinematicCharacterController::default())
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
}

fn log(query: Query<(&MoveToPoint, &Velocity)>) {
    for (&MoveToPoint(point), &Velocity { linvel, angvel }) in query.iter() {
        println!("{point}, {linvel}, {angvel}.");
    }
}
