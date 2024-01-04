use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    bird::Bird,
    pipes::{PipePart, PipeSection},
    scoring::Scoreboard,
};

#[derive(Default, Component)]
pub struct Collider;

#[derive(Default, Component, Deref, DerefMut, PartialEq)]
pub struct Velocity(Vec2);

impl Velocity {
    pub const fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Event, Default)]
pub struct CollisionEvent;

pub fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut bird_query: Query<(Entity, &mut Velocity, &Transform), With<Bird>>,
    collider_query: Query<(&Transform, &PipeSection), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (entity, mut _bird_velocity, bird_transform) = bird_query.single_mut();
    let ball_size = bird_transform.scale.truncate();

    // check collision with walls
    for (pipe_transform, pipe_section) in &collider_query {
        let collision = collide(
            bird_transform.translation,
            ball_size,
            pipe_transform.translation,
            pipe_transform.scale.truncate(),
        );
        if let Some(_collision) = collision {
            // Sends a collision event so that other systems can react to the collision
            collision_events.send_default();

            match pipe_section {
                PipeSection::Middle => scoreboard.score += 1,
                _ => Bird::kill_bird(&mut commands, entity),
            }

            // Bricks should be despawned and increment the scoreboard on collision
            // if maybe_brick.is_some() {
            //     // scoreboard.score += 1;
            //     commands.entity(collider_entity).despawn();
            // }

            // reflect the ball when it collides
            // let mut reflect_x = false;
            // let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            // match collision {
            //     Collision::Left => reflect_x = bird_velocity.x > 0.0,
            //     Collision::Right => reflect_x = bird_velocity.x < 0.0,
            //     Collision::Top => reflect_y = bird_velocity.y < 0.0,
            //     Collision::Bottom => reflect_y = bird_velocity.y > 0.0,
            //     Collision::Inside => { /* do nothing */ }
            // }

            // reflect velocity on the x-axis if we hit something on the x-axis
            // if reflect_x {
            //     bird_velocity.x = -bird_velocity.x;
            // }

            // reflect velocity on the y-axis if we hit something on the y-axis
            // if reflect_y {
            //     bird_velocity.y = -bird_velocity.y;
            // }
        }
    }
}
