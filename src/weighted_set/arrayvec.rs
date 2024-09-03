use super::{WeightedEntry, WeightedItem};
use arrayvec::ArrayVec;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

/// Implements WeightedCollection using a ArrayVec as the collection.
pub struct WeightedArrayVec<T, const CAP: usize> {
	pub(crate) total_weight: usize,

	//don't expose mutably
	pub(crate) array_vec: ArrayVec<WeightedEntry<T>, CAP>,
}

impl<T, const CAP: usize> WeightedArrayVec<T, CAP> {
	pub fn new() -> Self {
		Self {
			array_vec: ArrayVec::new(),
			total_weight: 0,
		}
	}
}

impl<T, const CAP: usize, U> AsRef<U> for WeightedArrayVec<T, CAP>
where
	<WeightedArrayVec<T, CAP> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: Debug, const CAP: usize> Debug for WeightedArrayVec<T, CAP> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WeightedVec")
			.field("total_weight", &self.total_weight)
			.field("array_vec", &self.array_vec)
			.finish()
	}
}

impl<T, const CAP: usize> Deref for WeightedArrayVec<T, CAP> {
	type Target = ArrayVec<WeightedEntry<T>, CAP>;

	fn deref(&self) -> &Self::Target {
		&self.array_vec
	}
}

impl<T, const CAP: usize> super::WeightedCollection<T> for WeightedArrayVec<T, CAP> {
	fn raffle(&self, partition_weight: usize) -> Option<&WeightedEntry<T>> {
		(&self.array_vec).get(self.array_vec.partition_point(|entry| entry.partition_weight < partition_weight))
	}

	fn total_weight(&self) -> usize {
		self.total_weight
	}
}

impl<T, const CAP: usize> super::WeightedCollectionMut<T> for WeightedArrayVec<T, CAP> {
	fn clear(&mut self) {
		self.total_weight = 0;

		self.array_vec.clear();
	}

	fn pop(&mut self) -> Option<WeightedItem<T>> {
		let entry = self.array_vec.pop()?;
		self.total_weight -= entry.weight.get();

		Some(entry.into())
	}

	fn push(&mut self, item: impl Into<WeightedItem<T>>) {
		let item = item.into();

		self.array_vec.push(WeightedEntry {
			value: item.value,
			weight: item.weight,
			partition_weight: self.total_weight,
		});
	}

	fn raffle_mut(&mut self, partition_weight: usize) -> Option<&mut WeightedEntry<T>> {
		let partition_point = self.array_vec.partition_point(|entry| entry.partition_weight < partition_weight);

		(&mut self.array_vec).get_mut(partition_point)
	}
}
