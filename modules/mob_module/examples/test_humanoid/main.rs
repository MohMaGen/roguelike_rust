use bevy::prelude::*;
use mob_module::mob::humanoid::HumanoidBundle;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, startup).run();
}

fn startup(mut commands: Commands, imgs: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let humanoid = HumanoidBundle::spawn_new(&mut commands, imgs.load("test_hero/hero_left.png"));
    commands.spawn(humanoid);
}
