use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, (print_ball_altitude, update_velocity))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(50.0, 500.0))
        .insert(TransformBundle::from(Transform::from_xyz(100.0, 0.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(GravityScale(0.0))
        .insert(Velocity::linear(Vec2::new(0.0, -10.0)))
        .insert(TransformBundle::from(Transform::from_xyz(-400.0, 0.0, 0.0)));
}

fn update_velocity(mut velocity: Query<&mut Velocity>) {
    for mut vel in velocity.iter_mut() {
        *vel = Velocity::linear(Vec2::new(10.0, 0.0));
    }
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
