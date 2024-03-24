use crate::sign::Sign;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use std::{
	io::stdin,
	sync::{
		mpsc::{channel, Receiver, Sender},
		Mutex,
	},
};

/// Type ID used for the StdinStringEvent type.
pub struct StdinPhantom;

/// The event for strings coming from the standard input.
pub type StdinStringEvent = Sign<String, StdinPhantom>;

/// The mpsc channel sender for transmitting strings from the stdin thread.
/// Typically used for sending non-empty strings that do not contain a line feed ('\n') character.
#[derive(Resource)]
pub struct StdinTx(Mutex<Sender<String>>);

#[derive(Resource)]
struct StdinRx(Mutex<Receiver<String>>);

#[allow(dead_code)]
impl StdinTx {
	/// Send a string to be processed in the standard input system.
	/// If you plan to send many, you may want to use the return value of `sender` instead.
	pub fn send(&self, string: String) {
		self.0.lock().unwrap().send(string).unwrap();
	}

	/// Returns a clone of the `Sender<String>` used in transmitting the standard input.
	pub fn sender(&self) -> Sender<String> {
		self.0.lock().unwrap().clone()
	}
}

/// A plugin for reading the standard input from within your bevy systems.
pub struct PluginMain<Schedule: ScheduleLabel + Clone = FixedFirst> {
	pub schedule: Schedule,
}

impl<Schedule: ScheduleLabel + Clone> PluginMain<Schedule> {
	pub fn new(schedule: Schedule) -> Self {
		Self { schedule }
	}
}

impl Default for PluginMain<FixedFirst> {
	fn default() -> Self {
		Self::new(FixedFirst)
	}
}

impl Plugin for PluginMain {
	fn build(&self, app: &mut App) {
		let (tx, rx) = channel::<String>();
		let handle_tx = tx.clone();

		std::thread::spawn(|| handle_stdin(handle_tx));

		app.add_event::<StdinStringEvent>()
			.add_systems(self.schedule.clone(), write_stdin_events)
			.insert_resource(StdinTx(Mutex::new(tx)))
			.insert_resource(StdinRx(Mutex::new(rx)));
	}
}

/// Reads the standard input on a separate thread.
fn handle_stdin(tx: Sender<String>) {
	let mut buffer = String::new();

	loop {
		stdin().read_line(&mut buffer).expect("stdin-thread failed to read stdin");
		buffer.pop();

		//don't send empty strings
		if buffer.is_empty() {
			continue;
		}

		tx.send(buffer.clone()).expect("stdin-thread failed to transmit line from stdin");
		buffer.clear();

		//if the buffer became big, shrink it back down
		if buffer.capacity() > 512 {
			buffer.shrink_to(512);
		}
	}
}

/// Bevy system which writes the strings received from `handle_stdin` into the `StdinStringEvent` event writer.
fn write_stdin_events(mut stdin_events: EventWriter<StdinStringEvent>, stdin_rx: Res<StdinRx>) {
	let guard = stdin_rx.as_ref().0.lock().unwrap();

	while let Ok(string) = guard.try_recv() {
		stdin_events.send(Sign::new(string));
	}
}
