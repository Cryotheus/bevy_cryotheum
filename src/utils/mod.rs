//! Miscellaneous utilities for usage in Bevy projects.

use std::hash::Hash;
use std::path::PathBuf;
use std::sync::LazyLock;
use bevy::input::ButtonInput;

pub mod collection_esoterics;
pub mod sign;
pub mod weighted_set;
pub mod registry;


/// Array where the index is an ASCII character's byte representation,
/// and the value is true if it is considered alpha numeric.
/// `A-Z`, `a-z`, `0-9`, and `_`
pub static ALPHA_NUMERIC_BYTES: LazyLock<[bool; 256]> = LazyLock::new(|| {
	let mut array = [false; 256];
	array[b'_' as usize] = true;

	for index in (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9') {
		array[index as usize] = true;
	}

	array
});

/// The current working directory.
pub static CWD: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().expect("Failed to find current working directory!"));

/// The directory which contains the running executable.
pub static EXE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
	let mut path = std::env::current_exe().expect("Failed to find executable path!");

	//we want the dir, not the executable itself
	path.pop();

	path
});

/// For implementing the [`CommonSign::common_sign`] method on various types, which are typically tuples.
/// Most notable implemented on `[bool; 2]`/`(bool, bool)` and `(&ButtonInput<T>, T, T)`
pub trait CommonSign {
	/// Returns 1f32, 0f32, or -1f32 as would be done commonly when making an arithmetic sign function of the given type/types.
	fn common_sign(&self) -> f32;
}

impl CommonSign for (bool, bool) {
	/// (false, true) => 1f32
	fn common_sign(&self) -> f32 {
		match self {
			(true, false) => -1.,
			(false, true) => 1.,
			(false, false) | (true, true) => 0.,
		}
	}
}

impl CommonSign for [bool; 2] {
	/// (false, true) => 1f32
	fn common_sign(&self) -> f32 {
		match self {
			[true, false] => -1.,
			[false, true] => 1.,
			[false, false] | [true, true] => 0.,
		}
	}
}

impl<'a, T> CommonSign for (&'a ButtonInput<T>, T, T)
where
	T: Copy + Eq + Hash + Send + Sync + 'static,
{
	fn common_sign(&self) -> f32 {
		(self.0.pressed(self.1), self.0.pressed(self.2)).common_sign()
	}
}

/// For implementing the `is_alpha_numeric` method on various types, which are typically strings.
pub trait IsAlphaNumeric {
	/// Comparse each byte against [`ALPHA_NUMERIC_BYTES`].
	fn is_alpha_numeric(&self) -> bool;
}

impl IsAlphaNumeric for str {
	fn is_alpha_numeric(&self) -> bool {
		self.bytes().all(|byte| ALPHA_NUMERIC_BYTES[byte as usize])
	}
}

impl IsAlphaNumeric for String {
	fn is_alpha_numeric(&self) -> bool {
		self.bytes().all(|byte| ALPHA_NUMERIC_BYTES[byte as usize])
	}
}

impl IsAlphaNumeric for u8 {
	fn is_alpha_numeric(&self) -> bool {
		ALPHA_NUMERIC_BYTES[*self as usize]
	}
}

impl IsAlphaNumeric for usize {
	fn is_alpha_numeric(&self) -> bool {
		ALPHA_NUMERIC_BYTES[*self]
	}
}
