use super::{Humanoid, HumanoidBody, HumanoidBundle, MobBundle};

impl HumanoidBundle {
    /// # Creates new humanoid bundle
    pub fn new(humanoid_body: HumanoidBody) -> Self {
        Self {
            humanoid_body,
            id: Humanoid,
            mob: MobBundle::default()
        }
    }
}
