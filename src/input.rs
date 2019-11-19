use winit::event::{DeviceEvent, ElementState, KeyboardInput, VirtualKeyCode};

use std::collections::HashSet;

#[derive(Default)]
pub struct InputState {
    keys_pressed: HashSet<VirtualKeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        let keys_pressed = HashSet::new();

        InputState { keys_pressed }
    }

    pub fn is_key_pressed(&self, code: VirtualKeyCode) -> bool {
        self.keys_pressed.contains(&code)
    }

    pub fn update(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(code),
                state,
                ..
            }) => match state {
                ElementState::Pressed => {
                    self.keys_pressed.insert(code.clone());
                }
                ElementState::Released => {
                    self.keys_pressed.remove(code);
                }
            },
            _ => {}
        }
    }
}
