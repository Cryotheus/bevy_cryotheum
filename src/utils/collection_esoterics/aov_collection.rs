use bevy::utils::HashMap;
use std::cmp::Ordering;
use std::collections::HashMap as StdHashMap;
use std::hash::Hash;
use std::mem;

/// "Array of Options as Value" collection.
/// Check stock implementers.
pub trait AovCollection<K, V, const SIZE: usize> {
	fn aov_contains(&self, index: usize, key: &K) -> bool {
		self.aov_get(index, key).is_some()
	}

	fn aov_get(&self, index: usize, key: &K) -> Option<&V> {
		self.aov_get_array(key)?[index].as_ref()
	}

	fn aov_get_array(&self, key: &K) -> Option<&[Option<V>; SIZE]>;
}

/// Mutable functions for the `AovCollection` trait.
pub trait AovCollectionMut<K, V, const SIZE: usize>: AovCollection<K, V, SIZE> {
	fn aov_get_mut(&mut self, index: usize, key: &K) -> Option<&mut V> {
		self.aov_get_array_mut(key)?[index].as_mut()
	}

	fn aov_get_array_mut(&mut self, key: &K) -> Option<&mut [Option<V>; SIZE]>;

	fn aov_insert(&mut self, index: usize, key: &K, value: V) -> Option<V>;

	fn aov_remove(&mut self, index: usize, key: &K) -> Option<V>;

	fn aov_remove_array(&mut self, key: &K) -> Option<[Option<V>; SIZE]>;
}

pub type AovHashMap<K, V, const SIZE: usize> = HashMap<K, [Option<V>; SIZE]>;

impl<K: Eq + Hash, V, const SIZE: usize> AovCollection<K, V, SIZE> for AovHashMap<K, V, SIZE> {
	fn aov_get_array(&self, key: &K) -> Option<&[Option<V>; SIZE]> {
		self.get(key)
	}
}

impl<K: Clone + Eq + Hash, V: Copy, const SIZE: usize> AovCollectionMut<K, V, SIZE> for AovHashMap<K, V, SIZE> {
	fn aov_get_array_mut(&mut self, key: &K) -> Option<&mut [Option<V>; SIZE]> {
		self.get_mut(key)
	}

	fn aov_insert(&mut self, index: usize, key: &K, value: V) -> Option<V> {
		if let Some(existing_array) = self.aov_get_array_mut(key) {
			return mem::replace(&mut existing_array[index], Some(value));
		}

		let mut array: [Option<V>; SIZE] = [None; SIZE];
		array[index] = Some(value);

		self.insert(key.clone(), array);

		None
	}

	fn aov_remove(&mut self, index: usize, key: &K) -> Option<V> {
		let array;

		let removed = if let Some(existing_array) = self.aov_get_array_mut(key) {
			let value = mem::replace(&mut existing_array[index], None); //array
			array = existing_array;

			value
		} else {
			return None;
		};

		if array.iter().all(|array_option| array_option.is_none()) {
			self.remove(key);
		}

		removed
	}

	fn aov_remove_array(&mut self, key: &K) -> Option<[Option<V>; SIZE]> {
		self.remove(key)
	}
}

pub type AovStdHashMap<K, V, const SIZE: usize> = StdHashMap<K, [Option<V>; SIZE]>;

impl<K: Eq + Hash, V, const SIZE: usize> AovCollection<K, V, SIZE> for AovStdHashMap<K, V, SIZE> {
	fn aov_get_array(&self, key: &K) -> Option<&[Option<V>; SIZE]> {
		self.get(key)
	}
}

impl<K: Clone + Eq + Hash, V: Copy, const SIZE: usize> AovCollectionMut<K, V, SIZE> for AovStdHashMap<K, V, SIZE> {
	fn aov_get_array_mut(&mut self, key: &K) -> Option<&mut [Option<V>; SIZE]> {
		self.get_mut(key)
	}

	fn aov_insert(&mut self, index: usize, key: &K, value: V) -> Option<V> {
		if let Some(existing_array) = self.aov_get_array_mut(key) {
			return mem::replace(&mut existing_array[index], Some(value));
		}

		let mut array: [Option<V>; SIZE] = [None; SIZE];
		array[index] = Some(value);

		self.insert(key.clone(), array);

		None
	}

	fn aov_remove(&mut self, index: usize, key: &K) -> Option<V> {
		let array;

		let removed = if let Some(existing_array) = self.aov_get_array_mut(key) {
			let value = mem::replace(&mut existing_array[index], None); //array
			array = existing_array;

			value
		} else {
			return None;
		};

		if array.iter().all(|array_option| array_option.is_none()) {
			self.remove(key);
		}

		removed
	}

	fn aov_remove_array(&mut self, key: &K) -> Option<[Option<V>; SIZE]> {
		self.remove(key)
	}
}

pub type AovVec<T, const SIZE: usize> = Vec<[Option<T>; SIZE]>;

impl<T, const SIZE: usize> AovCollection<usize, T, SIZE> for AovVec<T, SIZE> {
	fn aov_get_array(&self, key: &usize) -> Option<&[Option<T>; SIZE]> {
		self.get(*key)
	}
}

impl<T: Copy, const SIZE: usize> AovCollectionMut<usize, T, SIZE> for AovVec<T, SIZE> {
	fn aov_get_array_mut(&mut self, key: &usize) -> Option<&mut [Option<T>; SIZE]> {
		self.get_mut(*key)
	}

	/// This is O(n) time complexity when the index is higher than the vec's length.
	fn aov_insert(&mut self, index: usize, key: &usize, value: T) -> Option<T> {
		if let Some(existing_array) = self.aov_get_array_mut(key) {
			return mem::replace(&mut existing_array[index], Some(value));
		}

		let mut array: [Option<T>; SIZE] = [None; SIZE];

		//bridge the gap
		for _ in self.len()..index {
			//this will make a copy of the None-filled array
			self.push(array);
		}

		array[index] = Some(value);

		None
	}

	fn aov_remove(&mut self, index: usize, key: &usize) -> Option<T> {
		let removed = match self.aov_get_array_mut(key) {
			Some(existing_array) => mem::replace(&mut existing_array[index], None),
			None => return None,
		};

		let length = self.len();

		//if the index is the last entry in the vec
		if index == length - 1 {
			//shrink it down and remove the gap that may have been created by aov_insert
			for rev_index in (0..length).rev() {
				if self[rev_index].iter().all(|array_option| array_option.is_none()) {
					self.pop();
				} else {
					break;
				}
			}
		}

		removed
	}

	fn aov_remove_array(&mut self, key: &usize) -> Option<[Option<T>; SIZE]> {
		if self.is_empty() {
			return None;
		}

		let key = *key;
		let last_index = self.len() - 1;

		match key.cmp(&last_index) {
			Ordering::Less => Some(mem::replace(&mut self[key], [None; SIZE])),

			Ordering::Equal => {
				let removed = self.pop();

				for rev_index in (0..last_index).rev() {
					if self[rev_index].iter().all(|array_option| array_option.is_none()) {
						self.pop();
					} else {
						break;
					}
				}

				removed
			}

			Ordering::Greater => None,
		}
	}
}

#[cfg(feature = "arrayvec")]
pub mod arrayvec {
	use std::cmp::Ordering;
	use std::mem;
	use ::arrayvec::ArrayVec;

	pub type AovArrayVec<T, const SIZE: usize, const CAP: usize> = ArrayVec<[Option<T>; SIZE], CAP>;

	impl<T, const SIZE: usize, const CAP: usize> super::AovCollection<usize, T, SIZE> for AovArrayVec<T, SIZE, CAP> {
		fn aov_get_array(&self, key: &usize) -> Option<&[Option<T>; SIZE]> {
			self.get(*key)
		}
	}

	impl<T: Copy, const SIZE: usize, const CAP: usize> super::AovCollectionMut<usize, T, SIZE> for AovArrayVec<T, SIZE, CAP> {
		fn aov_get_array_mut(&mut self, key: &usize) -> Option<&mut [Option<T>; SIZE]> {
			self.get_mut(*key)
		}

		/// This is O(n) time complexity when the index is higher than the vec's length.
		fn aov_insert(&mut self, index: usize, key: &usize, value: T) -> Option<T> {
			if let Some(existing_array) = self.aov_get_array_mut(key) {
				return mem::replace(&mut existing_array[index], Some(value));
			}

			let mut array: [Option<T>; SIZE] = [None; SIZE];

			//bridge the gap
			for _ in self.len()..index {
				//this will make a copy of the None-filled array
				self.push(array);
			}

			array[index] = Some(value);

			None
		}

		fn aov_remove(&mut self, index: usize, key: &usize) -> Option<T> {
			let removed = match self.aov_get_array_mut(key) {
				Some(existing_array) => mem::replace(&mut existing_array[index], None),
				None => return None,
			};

			let length = self.len();

			//if the index is the last entry in the vec
			if index == length - 1 {
				//shrink it down and remove the gap that may have been created by aov_insert
				for rev_index in (0..length).rev() {
					if self[rev_index].iter().all(|array_option| array_option.is_none()) {
						self.pop();
					} else {
						break;
					}
				}
			}

			removed
		}

		fn aov_remove_array(&mut self, key: &usize) -> Option<[Option<T>; SIZE]> {
			if self.is_empty() {
				return None;
			}

			let key = *key;
			let last_index = self.len() - 1;

			match key.cmp(&last_index) {
				Ordering::Less => Some(mem::replace(&mut self[key], [None; SIZE])),

				Ordering::Equal => {
					let removed = self.pop();

					for rev_index in (0..last_index).rev() {
						if self[rev_index].iter().all(|array_option| array_option.is_none()) {
							self.pop();
						} else {
							break;
						}
					}

					removed
				}

				Ordering::Greater => None,
			}
		}
	}
}

#[cfg(feature = "arrayvec")]
pub use arrayvec::*;

#[cfg(feature = "smallvec")]
pub mod smallvec {
	use ::smallvec::SmallVec;
	use std::cmp::Ordering;
	use std::mem;

	pub type AovSmallVec<T, const SIZE: usize, const STACK: usize> = SmallVec<[[Option<T>; SIZE]; STACK]>;

	impl<T, const SIZE: usize, const STACK: usize> super::AovCollection<usize, T, SIZE> for AovSmallVec<T, SIZE, STACK> {
		fn aov_get_array(&self, key: &usize) -> Option<&[Option<T>; SIZE]> {
			self.get(*key)
		}
	}

	impl<T: Copy, const SIZE: usize, const STACK: usize> super::AovCollectionMut<usize, T, SIZE> for AovSmallVec<T, SIZE, STACK> {
		fn aov_get_array_mut(&mut self, key: &usize) -> Option<&mut [Option<T>; SIZE]> {
			self.get_mut(*key)
		}

		/// This is O(n) time complexity when the index is higher than the vec's length.
		fn aov_insert(&mut self, index: usize, key: &usize, value: T) -> Option<T> {
			if let Some(existing_array) = self.aov_get_array_mut(key) {
				return mem::replace(&mut existing_array[index], Some(value));
			}

			let mut array: [Option<T>; SIZE] = [None; SIZE];

			//bridge the gap
			for _ in self.len()..index {
				//this will make a copy of the None-filled array
				self.push(array);
			}

			array[index] = Some(value);

			None
		}

		fn aov_remove(&mut self, index: usize, key: &usize) -> Option<T> {
			let removed = match self.aov_get_array_mut(key) {
				Some(existing_array) => mem::replace(&mut existing_array[index], None),
				None => return None,
			};

			let length = self.len();

			//if the index is the last entry in the vec
			if index == length - 1 {
				//shrink it down and remove the gap that may have been created by aov_insert
				for rev_index in (0..length).rev() {
					if self[rev_index].iter().all(|array_option| array_option.is_none()) {
						self.pop();
					} else {
						break;
					}
				}
			}

			removed
		}

		fn aov_remove_array(&mut self, key: &usize) -> Option<[Option<T>; SIZE]> {
			if self.is_empty() {
				return None;
			}

			let key = *key;
			let last_index = self.len() - 1;

			match key.cmp(&last_index) {
				Ordering::Less => Some(mem::replace(&mut self[key], [None; SIZE])),

				Ordering::Equal => {
					let removed = self.pop();

					for rev_index in (0..last_index).rev() {
						if self[rev_index].iter().all(|array_option| array_option.is_none()) {
							self.pop();
						} else {
							break;
						}
					}

					removed
				}

				Ordering::Greater => None,
			}
		}
	}

}

#[cfg(feature = "smallvec")]
pub use smallvec::*;
