use bevy::prelude::*;

use crate::{
    input::{Action, InputHandler},
    physics::Velocity,
};

#[derive(Component)]
pub struct Bird;

impl Bird {
    const BIRD_SCALE: Vec3 = Vec3::new(0.25, 0.25, 0.25);
    const BIRD_SPAWN: Vec3 = Vec3::new(-535.0, 0.0, 0.0);
    const START_TRANSFORM: Transform =
        Transform::from_translation(Self::BIRD_SPAWN).with_scale(Self::BIRD_SCALE);

    const GRAVITY: f32 = -9.8;
    const JUMP_POWER: f32 = 45.0 * 5.0;
    const SPEED_LIMIT: f32 = Self::JUMP_POWER * 2.0;

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("bevy.png"),
                transform: Self::START_TRANSFORM,
                ..default()
            },
            Velocity::default(),
            Bird,
        ));
    }

    pub fn physics(
        time: Res<Time>,
        mouse_buttons: Res<Input<MouseButton>>,
        keyboard: Res<Input<KeyCode>>,
        mut bird: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    ) {
        let (mut transform, mut velocity) = bird.single_mut();

        let handler = InputHandler::new(Some(&mouse_buttons), Some(&keyboard), None);

        if handler.handle_action(Action::JUMP) {
            // If the bird was previously going down, set its velocity to 0
            // This gives greater control over the birds movement
            if velocity.y < 0.0 {
                velocity.y = 0.0;
            }

            velocity.y += Self::JUMP_POWER;
            // If bird travels too fast, just maintain the speed limit
            if velocity.y >= Self::SPEED_LIMIT {
                velocity.y = Self::SPEED_LIMIT;
            }
        } else {
            velocity.y += Self::GRAVITY;
        }

        transform.translation.y += velocity.y * time.delta_seconds();
    }

    pub fn kill_bird(commands: &mut Commands, entity: Entity) {
        unimplemented!("Kill bird...");
    }
}

impl super::SystemsController for Bird {
    fn include_systems(app: &mut App) -> &mut App {
        app.add_systems(Startup, Bird::setup.after(super::setup))
            .add_systems(FixedUpdate, Bird::physics)
    }
}
