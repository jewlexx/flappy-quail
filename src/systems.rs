use bevy::prelude::*;

pub trait SystemsController {
    fn include_systems(app: &mut App) -> &mut App;
}

pub trait SystemAdder {
    fn include_systems<C: SystemsController>(&mut self);
}

impl SystemAdder for App {
    fn include_systems<C: SystemsController>(&mut self) {
        C::include_systems(self);
    }
}
