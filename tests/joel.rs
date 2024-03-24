use bevy_cryotheum::plugins::joel::Joel;
use std::str::FromStr;

#[test]
fn joel_structs() {
	let control = Joel {
		arguments: vec![String::from("Screen-space Bloom"), String::from("true")],
		command: "config_set".into(),
	};

	assert_eq!(control, Joel::from_str("config_set 'Screen-space Bloom' true").unwrap());
	assert_eq!(control, Joel::from_str("config_set Screen-space\\ Bloom true").unwrap());
	assert_eq!(control, Joel::from_str("config_set \"Screen-space Bloom\" \"true\"").unwrap());
	assert_eq!(control, Joel::from_str("config_set \"Screen-space\\ Bloom\" \"true\"").unwrap());
}
