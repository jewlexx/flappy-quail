use bevy::prelude::*;

mod bird;
mod input;

use bird::Bird;

mod world_config {
    pub const GRAVITY: f32 = -9.8;
    pub const JUMP_POWER: f32 = 45.0 * 5.0;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, Bird::setup.after(setup))
        .add_systems(FixedUpdate, Bird::physics)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
