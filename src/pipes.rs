use bevy::prelude::*;

use crate::physics::{Collider, Velocity};

#[derive(Bundle)]
pub struct Pipes {
    top: PipePart,
    bottom: PipePart,
    middle: PipePart,
}

impl Pipes {
    pub const VELOCITY: Velocity = Velocity::new(-100.0, 0.0);

    pub fn new(mut commands: Commands, asset_server: Res<AssetServer>) -> Self {
        Self {
            top: PipePart::new(asset_server.as_ref(), PipeSection::Top),
            bottom: PipePart::new(asset_server.as_ref(), PipeSection::Bottom),
            middle: PipePart::new(asset_server.as_ref(), PipeSection::Middle),
        }
    }
}

#[derive(Bundle)]
pub struct PipePart {
    sprite: SpriteBundle,
    collider: Collider,
    transform: Transform,
    section: PipeSection,
}

impl PipePart {
    const OFFSET: f32 = 0.0;

    pub fn new(asset_server: &AssetServer, section: PipeSection) -> Self {
        let sprite = match section {
            PipeSection::Top => SpriteBundle {
                texture: asset_server.load("pipe_top.png"),
                ..Default::default()
            },
            PipeSection::Middle => todo!(),
            PipeSection::Bottom => todo!(),
        };

        // let transform = match section {
        //     // PipeSection::Top => Transform::from_translation(Vec3::new(
        //     //     0.0,
        //     //     sprite.sprite..y / 2.0 + Self::OFFSET,
        //     //     0.0,
        //     // )),
        //     // PipeSection::Middle => todo!(),
        //     // PipeSection::Bottom => todo!(),
        // };

        Self {
            sprite,
            collider: Collider,
            transform: todo!(),
            section,
        }
    }
}

impl PipePart {
    pub fn section(&self) -> PipeSection {
        self.section
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub enum PipeSection {
    Top,
    Middle,
    Bottom,
}
