use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct InputHandler<'a> {
    mouse_buttons: Option<&'a Input<MouseButton>>,
    keyboard: Option<&'a Input<KeyCode>>,
    gamepad: Option<&'a Input<GamepadButton>>,
}

impl<'a> InputHandler<'a> {
    pub fn new(
        mouse_buttons: Option<&'a Input<MouseButton>>,
        keyboard: Option<&'a Input<KeyCode>>,
        gamepad: Option<&'a Input<GamepadButton>>,
    ) -> Self {
        Self {
            mouse_buttons,
            keyboard,
            gamepad,
        }
    }

    pub fn handle_action(&self, action: Action) -> bool {
        let inputs = action.into_tuple();

        if let Some(keyboard) = inputs.0 {
            if let Some(input) = self.keyboard {
                if input.just_pressed(keyboard) {
                    return true;
                }
            }
        }

        if let Some(mouse) = inputs.1 {
            if let Some(input) = self.mouse_buttons {
                if input.just_pressed(mouse) {
                    return true;
                }
            }
        }

        if let Some(gamepad) = inputs.2 {
            if let Some(input) = self.gamepad {
                if input.just_pressed(gamepad) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Action {
    keyboard: Option<KeyCode>,
    mouse: Option<MouseButton>,
    gamepad: Option<GamepadButton>,
}

impl Action {
    pub const JUMP: Self = Self {
        keyboard: Some(KeyCode::Space),
        mouse: Some(MouseButton::Left),
        gamepad: None,
    };

    pub const fn into_tuple(self) -> (Option<KeyCode>, Option<MouseButton>, Option<GamepadButton>) {
        (self.keyboard, self.mouse, self.gamepad)
    }
}
