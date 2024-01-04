use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Scoreboard {
    pub score: usize,
}
