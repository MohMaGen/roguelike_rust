use bevy::ecs::{bundle::Bundle, component::Component, entity::Entity};

use super::MobBundle;

mod new;

#[derive(Bundle, Clone, Debug, PartialEq)]
pub struct HumanoidBundle {
    pub id: Humanoid,

    pub humanoid_body: HumanoidBody,

    pub mob: MobBundle,
}

#[derive(Component, Clone, Debug, PartialEq, Default)]
pub struct Humanoid;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct HumanoidBody {
    pub body: Entity,
    pub head: Entity,
    pub left_hand: Entity,
    pub left_leg: Entity,
    pub right_hand: Entity,
    pub right_leg: Entity,
}
