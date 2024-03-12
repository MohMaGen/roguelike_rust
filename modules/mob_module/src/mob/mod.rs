use bevy::{ecs::component::Component, prelude::Bundle};
use game_utils::helper_bundles::GraphicParentBundle;

#[derive(Bundle)]
pub struct MobBundle {
    pub id: Mob,

    pub graphic_parent_bundle: GraphicParentBundle,
}

#[derive(Component)]
pub struct Mob;
