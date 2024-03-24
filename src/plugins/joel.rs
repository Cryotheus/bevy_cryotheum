use super::stdin::StdinStringEvent;
use anyhow::{anyhow, bail};
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use std::{mem::replace, str::FromStr};

#[derive(Clone, Debug, Event, PartialEq)]
pub struct Joel {
	/// The arguments this command was run with.
	pub arguments: Vec<String>,

	/// The command name used in comparison.
	pub command: String,

	/// The original string that was used in making the arguments vector.
	pub original_arguments: Option<String>,
}

impl Joel {
	/// Create a Joel with no arguments.
	pub fn new(command: impl Into<String>) -> Self {
		Self {
			arguments: Vec::new(),
			command: command.into(),
			original_arguments: None,
		}
	}

	/// Same as calling `len` on the `arguments` field.
	pub fn len(&self) -> usize {
		self.arguments.len()
	}

	/// Gets the original_arguments field, or rebuilds an equivalent.
	pub fn arguments_string(&self) -> String {
		if let Some(original_arguments) = &self.original_arguments {
			return original_arguments.clone();
		}

		let mut arguments_string = String::new();

		for argument in &self.arguments {
			let start_index = arguments_string.len();
			let mut has_whitespace = false;

			for char in argument.chars() {
				match char {
					'"' | '\'' | '\\' => arguments_string.push('\\'),

					_ => {
						if char.is_whitespace() {
							has_whitespace = true;
						}
					}
				}

				arguments_string.push(char);
			}

			//wrap in quotes
			if has_whitespace {
				arguments_string.insert(start_index, '"');
				arguments_string.push('"');
			}

			arguments_string.push(' ');
		}

		arguments_string.pop();
		arguments_string.shrink_to_fit();

		arguments_string
	}

	/// Attempt to parse the argument at the specified index.
	pub fn parse<T: FromStr>(&self, index: usize) -> anyhow::Result<T> {
		let argument = self.arguments.get(index).ok_or(anyhow!("Argument does not exist at index {index}."))?;

		T::from_str(argument).map_err(|_| anyhow!("Failed to parse argument #{index}. Argument: `{argument}`"))
	}

	pub fn push(&mut self, argument: String) {
		self.arguments.push(argument);
	}
}

impl PartialEq<&str> for Joel {
	fn eq(&self, other: &&str) -> bool {
		//self.arguments == other.arguments && self.command == other.command
		self.command.eq(other)
	}
}

impl FromStr for Joel {
	type Err = anyhow::Error;

	fn from_str(mut string: &str) -> Result<Self, Self::Err> {
		string = string.trim();
		let (command, arguments_string) = string.split_once(" ").unwrap_or((string, ""));
		let arguments_string = arguments_string.trim();

		if command.is_empty() {
			bail!("Empty command string");
		}

		let mut argument_builder = String::new();
		let mut joel = Joel::new(command);
		let mut state = JoelParseState::Natural;
		joel.original_arguments = Some(String::from(string));

		'chars: for char in arguments_string.chars() {
			use JoelParseState as Jps;

			if state == Jps::ExpectWhitespace {
				if char.is_whitespace() {
					if !argument_builder.is_empty() {
						joel.push(replace(&mut argument_builder, String::new()));
					}

					state = Jps::Natural;

					continue;
				} else {
					bail!("Expected whitespace following argument #{}", joel.len());
				}
			}

			match char {
				//escape character
				'\\' => {
					if !state.finish_escape() {
						state.start_escape();

						continue;
					}
				}

				//quotes and apostrophes are delimiters
				delimiter @ ('"' | '\'') => 'delimiter: {
					if state.finish_escape() {
						break 'delimiter;
					}

					if let Jps::Delimiter(existing_delimiter) = state {
						if delimiter == existing_delimiter {
							//exit the delimiter
							state = Jps::ExpectWhitespace;

							continue 'chars;
						}
					} else {
						//enter a delimiter
						state = Jps::Delimiter(delimiter);

						if !argument_builder.is_empty() {
							bail!("Malformed delimiter {delimiter} in argument {}", joel.len());
						}

						continue 'chars;
					}
				}

				char if char.is_whitespace() => match state {
					Jps::Delimiter(_) | Jps::Escape(_) => {}
					Jps::ExpectWhitespace => unreachable!(),

					Jps::Natural => {
						if !argument_builder.is_empty() {
							joel.push(replace(&mut argument_builder, String::new()));
						}

						continue;
					}
				},

				_ => {}
			}

			argument_builder.push(char);
			state.finish_escape();
		}

		if state.escaping() {
			bail!("Unfinished escape in arguments");
		}

		if !argument_builder.is_empty() {
			joel.push(argument_builder);
		}

		Ok(joel)
	}
}

impl ToString for Joel {
	fn to_string(&self) -> String {
		format!("{} {}", self.command, self.arguments_string())
	}
}

//this should be made into a macro
impl<T0: FromStr, T1: FromStr> Into<anyhow::Result<(T0, T1)>> for Joel {
	fn into(self) -> anyhow::Result<(T0, T1)> {
		Ok((self.parse::<T0>(0)?, self.parse::<T1>(1)?))
	}
}

impl<T0: FromStr, T1: FromStr, T2: FromStr> Into<anyhow::Result<(T0, T1, T2)>> for Joel {
	fn into(self) -> anyhow::Result<(T0, T1, T2)> {
		Ok((self.parse::<T0>(0)?, self.parse::<T1>(1)?, self.parse::<T2>(1)?))
	}
}

impl<T0: FromStr, T1: FromStr, T2: FromStr, T3: FromStr> Into<anyhow::Result<(T0, T1, T2, T3)>> for Joel {
	fn into(self) -> anyhow::Result<(T0, T1, T2, T3)> {
		Ok((self.parse::<T0>(0)?, self.parse::<T1>(1)?, self.parse::<T2>(1)?, self.parse::<T3>(1)?))
	}
}

/// Used by the `Joel` struct for its `FromStr` trait implementation.
#[derive(Clone, Default, PartialEq)]
enum JoelParseState {
	/// In a delimiter of the contained char.
	Delimiter(char),

	/// Processing an escaped character.
	/// Contains the previous state.
	Escape(Box<JoelParseState>),

	/// The next char must be whitespace.
	ExpectWhitespace,

	/// Normal text processing.
	/// This is the default.
	#[default]
	Natural,
}

#[allow(dead_code)]
impl JoelParseState {
	pub fn escaping(&self) -> bool {
		match self {
			Self::Escape(_) => true,
			_ => false,
		}
	}

	/// Returns `true` if we were escaping, and reverts the state to its previous value.
	pub fn finish_escape(&mut self) -> bool {
		if let Self::Escape(boxed_previous) = self {
			*self = boxed_previous.as_mut().clone();

			true
		} else {
			false
		}
	}

	pub fn start_escape(&mut self) {
		*self = Self::Escape(Box::new(self.clone()));
	}
}

pub struct PluginMain<Schedule: ScheduleLabel + Clone = FixedPreUpdate> {
	/// Schedule for running the `stdin_to_joel` system.
	/// `None` means to not add the system.
	pub stdin_schedule: Option<Schedule>,
}

impl<Schedule: ScheduleLabel + Clone> PluginMain<Schedule> {
	pub fn new(stdin_schedule: Option<Schedule>) -> Self {
		Self { stdin_schedule }
	}
}

impl Default for PluginMain<FixedPreUpdate> {
	fn default() -> Self {
		Self::new(Some(FixedPreUpdate))
	}
}

/// Small command system that parses strings into a command and a vector of arguments.
impl Plugin for PluginMain {
	fn build(&self, app: &mut App) {
		app.add_event::<Joel>();

		if let Some(schedule) = &self.stdin_schedule {
			app.add_systems(schedule.clone(), stdin_to_joel);
		}
	}
}

/// Run condition for when there is a JOEL command queued.
pub fn run_if_joel_queued(joel_events: EventReader<Joel>) -> bool {
	!joel_events.is_empty()
}

/// Reads strings from the stdin plugin and attempts to convert them into JOEL commands.
/// # Panics
/// Requires the `stdin` plugin to function, otherwise will panic.
/// If the `read_stdin_events` field is `true`, this system is automatically added.
pub fn stdin_to_joel(mut joel_events: EventWriter<Joel>, mut stdin_string_events: EventReader<StdinStringEvent>) {
	for stdin_string in stdin_string_events.read() {
		match Joel::from_str(stdin_string) {
			Ok(joel) => {
				println!("Command `{}` received.", joel.command);
				joel_events.send(joel);
			}

			Err(error) => println!("Failed to parse JOEL command arguments.\n{error:#?}"),
		}
	}
}
