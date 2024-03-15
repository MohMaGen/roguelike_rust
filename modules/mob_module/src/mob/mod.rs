use bevy::{ecs::component::Component, prelude::Bundle};
use game_utils::helper_bundles::GraphicParentBundle;

pub mod humanoid;
pub mod body;
pub mod player;


#[derive(Bundle, Default, Clone, PartialEq, Debug)]
pub struct MobBundle {
    pub id: Mob,

    pub graphic_parent_bundle: GraphicParentBundle,
}

/// # Component identifier for all mobs.
#[derive(Component, Default, Clone, PartialEq, Debug)]
pub struct Mob;
