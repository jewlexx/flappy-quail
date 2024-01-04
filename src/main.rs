use bevy::prelude::*;

mod bird;
mod input;
mod physics;
mod pipes;
mod scoring;
mod systems;

use physics::CollisionEvent;
use scoring::Scoreboard;
pub(crate) use systems::*;

use bird::Bird;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.insert_resource(Scoreboard::default());

    app.add_systems(Startup, setup);
    app.include_systems::<Bird>();
    app.add_systems(FixedUpdate, physics::check_for_collisions);

    app.add_event::<CollisionEvent>();

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
