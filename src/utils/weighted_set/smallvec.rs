use super::{WeightedEntry, WeightedItem};
use smallvec::SmallVec;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Implements WeightedCollection using a SmallVec as the collection.
pub struct WeightedSmallVec<T, const SIZE: usize> {
	pub(crate) total_weight: usize,

	//don't expose mutably
	pub(crate) small_vec: SmallVec<[WeightedEntry<T>; SIZE]>,
}

impl<T, const CAP: usize> WeightedSmallVec<T, CAP> {
	pub fn new() -> Self {
		Self {
			small_vec: SmallVec::new(),
			total_weight: 0,
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			small_vec: SmallVec::with_capacity(capacity),
			total_weight: 0,
		}
	}

	/// `Vec::reserve`.
	pub fn reserve(&mut self, additional: usize) {
		self.small_vec.reserve(additional)
	}

	/// `Vec::reserve_exact`.
	pub fn reserve_exact(&mut self, additional: usize) {
		self.small_vec.reserve_exact(additional)
	}
}

impl<T, const SIZE: usize, U> AsRef<U> for WeightedSmallVec<T, SIZE>
where
	<WeightedSmallVec<T, SIZE> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: Debug, const SIZE: usize> Debug for WeightedSmallVec<T, SIZE> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WeightedVec")
			.field("total_weight", &self.total_weight)
			.field("small_vec", &self.small_vec)
			.finish()
	}
}

impl<T, const SIZE: usize> Deref for WeightedSmallVec<T, SIZE> {
	type Target = SmallVec<[WeightedEntry<T>; SIZE]>;

	fn deref(&self) -> &Self::Target {
		&self.small_vec
	}
}

impl<T, const SIZE: usize> super::WeightedCollection<T> for WeightedSmallVec<T, SIZE> {
	fn raffle(&self, partition_weight: usize) -> Option<&WeightedEntry<T>> {
		(&self.small_vec).get(self.small_vec.partition_point(|entry| entry.partition_weight < partition_weight))
	}

	fn total_weight(&self) -> usize {
		self.total_weight
	}
}

impl<T, const SIZE: usize> super::WeightedCollectionMut<T> for WeightedSmallVec<T, SIZE> {
	fn clear(&mut self) {
		self.total_weight = 0;

		self.small_vec.clear();
	}

	fn pop(&mut self) -> Option<WeightedItem<T>> {
		let entry = self.small_vec.pop()?;
		self.total_weight -= entry.weight.get();

		Some(entry.into())
	}

	fn push(&mut self, item: impl Into<WeightedItem<T>>) {
		let item = item.into();

		self.small_vec.push(WeightedEntry {
			value: item.value,
			weight: item.weight,
			partition_weight: self.total_weight,
		});
	}

	fn raffle_mut(&mut self, partition_weight: usize) -> Option<&mut WeightedEntry<T>> {
		let partition_point = self.small_vec.partition_point(|entry| entry.partition_weight < partition_weight);

		(&mut self.small_vec).get_mut(partition_point)
	}
}
