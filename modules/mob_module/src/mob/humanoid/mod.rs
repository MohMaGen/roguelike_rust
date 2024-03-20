use bevy::ecs::{bundle::Bundle, component::Component, entity::Entity};

use super::MobBundle;

mod new;

#[derive(Bundle, Clone, Debug, PartialEq)]
pub struct HumanoidBundle {
    pub id: Humanoid,

    pub body: HumanoidBody,
    pub mob: MobBundle,
}

#[derive(Component, Clone, Debug, PartialEq, Default)]
pub struct Humanoid;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct HumanoidBody(pub Entity);
