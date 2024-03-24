use bevy_cryotheum::plugins::joel::Joel;
use std::str::FromStr;

#[test]
fn joel_structs() {
	let control = Joel {
		arguments: vec![String::from("Screen-space Bloom"), String::from("true")],
		command: "config_set".into(),
		original_arguments: None,
	};

	//FromStr trait
	assert_eq!(control, Joel::from_str("config_set 'Screen-space Bloom' true").unwrap());
	assert_eq!(control, Joel::from_str("config_set Screen-space\\ Bloom true").unwrap());
	assert_eq!(control, Joel::from_str("config_set \"Screen-space Bloom\" \"true\"").unwrap());
	assert_eq!(control, Joel::from_str("config_set \"Screen-space\\ Bloom\" \"true\"").unwrap());

	//ToString trait
	assert_eq!(control.to_string(), String::from("config_set \"Screen-space Bloom\" true"));

	//PartialEq<&str> trait
	assert_eq!(control, "config_set");
}
