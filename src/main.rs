use bevy::prelude::*;

mod bird;
mod input;
mod physics;
mod pipes;
mod systems;

pub(crate) use systems::*;

use bird::Bird;

mod world_config {
    pub const GRAVITY: f32 = -9.8;
    pub const JUMP_POWER: f32 = 45.0 * 5.0;
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, setup);
    app.add_systems(FixedUpdate, physics::check_for_collisions);

    app.include_systems::<Bird>();

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
