use bevy::{
    ecs::bundle::Bundle,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    transform::components::{GlobalTransform, Transform},
};

///
/// # Bundle that sayes bevy to render entity and children.
///
/// if parant entity havn't texture, you can use this bundle
/// instead of SpriteBundle with null handler of texture.
///
#[derive(Bundle, Debug, Clone, Default, PartialEq)]
pub struct GraphicParentBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}
