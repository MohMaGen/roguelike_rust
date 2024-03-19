use bevy::app::PluginGroupBuilder;

pub mod move_to_entity;
pub mod move_to_point;
pub mod speed;

#[derive(Default)]
pub struct MovementPlugins;

impl bevy::prelude::PluginGroup for MovementPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(move_to_point::MoveToPointPlugin)
            .add(move_to_entity::MoveToEntityPlugin)
    }
}
