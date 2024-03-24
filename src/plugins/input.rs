use crate::sign::Sign;

use bevy::{
	input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
	prelude::*,
};

/// This type is solely used to create a type id that differentiates ButtonInput types.
/// This lets us have multiple different ButtonInput resources.
pub struct FixedInput;

/// `ButtonInput<T>` but with a different type ID to differentiate it from other resources of the same type ID.
pub type FixedButtonInput<T> = Sign<ButtonInput<T>, FixedInput>;

pub struct PluginMain;

impl Plugin for PluginMain {
	fn build(&self, app: &mut App) {
		app.add_systems(FixedFirst, build_fixed_input)
			.init_resource::<FixedButtonInput<KeyCode>>()
			.init_resource::<FixedButtonInput<MouseButton>>();
	}
}

fn build_fixed_input(
	mut key_code_input: ResMut<FixedButtonInput<KeyCode>>,
	mut keyboard_input_events: EventReader<KeyboardInput>,
	mut mouse_button_input: ResMut<FixedButtonInput<MouseButton>>,
	mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
	use bevy::input::ButtonState::*;

	//key code
	key_code_input.clear();

	for event in keyboard_input_events.read() {
		match event.state {
			Pressed => key_code_input.press(event.key_code),
			Released => key_code_input.release(event.key_code),
		}
	}

	//mouse buttons
	mouse_button_input.clear();

	for event in mouse_button_input_events.read() {
		match event.state {
			Pressed => mouse_button_input.press(event.button),
			Released => mouse_button_input.release(event.button),
		}
	}
}
