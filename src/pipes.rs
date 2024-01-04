use bevy::prelude::*;

use crate::physics::{Collider, Velocity};

#[derive(Component)]
pub struct Pipes {
    top: PipePart,
    bottom: PipePart,
    middle: PipePart,
    transform: Transform,
    velocity: Velocity,
}

#[derive(Component)]
pub struct PipePart {
    sprite: Sprite,
    collider: Collider,
    section: PipeSection,
}

impl PipePart {
    pub fn section(&self) -> PipeSection {
        self.section
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeSection {
    Top,
    Middle,
    Bottom,
}
