use bevy::{ecs::bundle::Bundle, sprite::SpriteBundle};

#[derive(Bundle, Default, Clone)]
pub struct BodyPartBundle {

    pub sprite: SpriteBundle,
}

pub struct BodyPart {
    
}
