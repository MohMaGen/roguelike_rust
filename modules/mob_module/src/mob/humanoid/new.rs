use bevy::{
    asset::Handle,
    ecs::system::Commands,
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
};

use crate::mob::MobBundle;

use super::{HumanoidBody, HumanoidBundle};

impl HumanoidBundle {
    pub fn spawn_new(commands: &mut Commands, body_texture: Handle<Image>) -> HumanoidBundle {
        let body = commands
            .spawn(SpriteBundle {
                transform: Transform::from_xyz(0., 0., 0.),
                texture: body_texture,
                ..Default::default()
            })
            .id();

        HumanoidBundle {
            id: super::Humanoid,
            body: HumanoidBody(body),
            mob: MobBundle::default(),
        }
    }
}
