use bevy::ecs::component::Component;

#[derive(Component, Debug, Default, PartialEq, PartialOrd)]
pub struct MoveSpeed(pub f32);
