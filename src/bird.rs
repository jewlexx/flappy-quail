use bevy::prelude::*;

use crate::world_config;

#[derive(Component)]
pub struct Bird {
    velocity: f32,
    jump_power: f32,
    gravity: f32,
    speed_limit: f32,
}

impl Bird {
    const BIRD_SCALE: Vec3 = Vec3::new(0.25, 0.25, 0.25);
    const BIRD_SPAWN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    const START_TRANSFORM: Transform =
        Transform::from_translation(Self::BIRD_SPAWN).with_scale(Self::BIRD_SCALE);

    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("bevy.png"),
                transform: Self::START_TRANSFORM,
                ..default()
            },
            Self {
                jump_power: world_config::JUMP_POWER,
                gravity: world_config::GRAVITY,
                speed_limit: world_config::JUMP_POWER * 2.0,
                velocity: default(),
            },
        ));
    }

    pub fn physics(
        time: Res<Time>,
        mouse_buttons: Res<Input<MouseButton>>,
        keyboard: Res<Input<KeyCode>>,
        mut bird: Query<(&mut Transform, &mut Bird)>,
    ) {
        let (mut transform, mut bird) = bird.single_mut();

        if mouse_buttons.just_pressed(MouseButton::Left) || keyboard.just_pressed(KeyCode::Space) {
            // If the bird was previously going down, set its velocity to 0
            // This gives greater control over the birds movement
            if bird.velocity < 0.0 {
                bird.velocity = 0.0;
            }

            bird.velocity += bird.jump_power;
            // If bird travels too fast, just maintain the speed limit
            if bird.velocity >= bird.speed_limit {
                bird.velocity = bird.speed_limit;
            }
        } else {
            bird.velocity += bird.gravity;
        }

        transform.translation.y += bird.velocity * time.delta_seconds();
    }
}
