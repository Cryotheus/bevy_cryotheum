#[cfg(feature = "arrayvec")]
pub mod arrayvec;

#[cfg(feature = "arrayvec")]
pub use crate::weighted_set::arrayvec::*;

#[cfg(feature = "smallvec")]
pub mod smallvec;

#[cfg(feature = "smallvec")]
pub use crate::weighted_set::smallvec::*;

use std::fmt::{Debug, Formatter};
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};

/// Implemented by `WeightedVec`.
pub trait WeightedCollection<T> {
	/// Returns a reference to an entry based on a supplied weight.
	/// Returns `None` if the supplied weight is >= `total_weight`.
	fn raffle(&self, partition_weight: usize) -> Option<&WeightedEntry<T>>;

	fn total_weight(&self) -> usize;
}

pub trait WeightedCollectionMut<T>: WeightedCollection<T> {
	fn clear(&mut self);

	fn pop(&mut self) -> Option<WeightedItem<T>>;

	fn push(&mut self, item: impl Into<WeightedItem<T>>);

	fn raffle_mut(&mut self, partition_weight: usize) -> Option<&mut WeightedEntry<T>>;
}

#[derive(Debug, thiserror::Error)]
pub enum WeightedCollectionError {
	#[error("Weight must not be zero in this context.")]
	ZeroWeight,
}

/// A value, weight, and a partition weight.
/// This is the internal collection item of a WeightedCollection implementer.
/// It is safe to assume that references of this type are in a weighted ollection.
pub struct WeightedEntry<T> {
	pub(crate) value: T,
	pub(crate) weight: NonZeroUsize,

	/// The weight at which to partition the collection in order to get this entry.
	pub(crate) partition_weight: usize,
}

impl<T> WeightedEntry<T> {
	pub fn into_inner(self) -> T {
		self.value
	}

	pub fn into_partition_weight(self) -> usize {
		self.partition_weight
	}

	pub fn into_weight(self) -> NonZeroUsize {
		self.weight
	}
}

impl<T, U> AsMut<U> for WeightedEntry<T>
where
	<WeightedEntry<T> as Deref>::Target: AsMut<U>,
{
	fn as_mut(&mut self) -> &mut U {
		self.deref_mut().as_mut()
	}
}

impl<T, U> AsRef<U> for WeightedEntry<T>
where
	<WeightedEntry<T> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: Debug> Debug for WeightedEntry<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WeightedEntry")
			.field("value", &self.value)
			.field("weight", &self.weight)
			.field("partition_weight", &self.partition_weight)
			.finish()
	}
}

impl<T> Deref for WeightedEntry<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> DerefMut for WeightedEntry<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}

/// A value and a weight.
pub struct WeightedItem<T> {
	pub(crate) value: T,
	pub(crate) weight: NonZeroUsize,
}

impl<T> WeightedItem<T> {
	pub fn into_inner(self) -> T {
		self.value
	}

	pub fn into_weight(self) -> NonZeroUsize {
		self.weight
	}

	pub fn new(value: T, weight: NonZeroUsize) -> Self {
		Self { value, weight }
	}
}

impl<T, U> AsMut<U> for WeightedItem<T>
where
	<WeightedItem<T> as Deref>::Target: AsMut<U>,
{
	fn as_mut(&mut self) -> &mut U {
		self.deref_mut().as_mut()
	}
}

impl<T, U> AsRef<U> for WeightedItem<T>
where
	<WeightedItem<T> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: Clone> Clone for WeightedItem<T> {
	fn clone(&self) -> Self {
		Self {
			value: self.value.clone(),
			weight: self.weight,
		}
	}
}

impl<T: Debug> Debug for WeightedItem<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WeightedItem")
			.field("value", &self.value)
			.field("weight", &self.weight)
			.finish()
	}
}

impl<T: Default> Default for WeightedItem<T> {
	fn default() -> Self {
		Self {
			value: T::default(),
			weight: NonZeroUsize::MIN,
		}
	}
}

impl<T> Deref for WeightedItem<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> DerefMut for WeightedItem<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}

impl<T> From<T> for WeightedItem<T> {
	fn from(value: T) -> Self {
		Self {
			value,
			weight: NonZeroUsize::MIN,
		}
	}
}

impl<T> From<WeightedEntry<T>> for WeightedItem<T> {
	fn from(entry: WeightedEntry<T>) -> Self {
		Self {
			value: entry.value,
			weight: entry.weight,
		}
	}
}

impl<T> From<(T, NonZeroUsize)> for WeightedItem<T> {
	fn from((value, weight): (T, NonZeroUsize)) -> Self {
		Self { value, weight }
	}
}

impl<T> TryFrom<(T, usize)> for WeightedItem<T> {
	type Error = WeightedCollectionError;

	fn try_from((value, weight): (T, usize)) -> Result<Self, Self::Error> {
		Ok(Self {
			value,
			weight: NonZeroUsize::new(weight).ok_or(WeightedCollectionError::ZeroWeight)?,
		})
	}
}

/// Implements WeightedCollection using a Vec as the collection.
pub struct WeightedVec<T> {
	pub(crate) total_weight: usize,

	//don't expose mutably
	pub(crate) vec: Vec<WeightedEntry<T>>,
}

impl<T> WeightedVec<T> {
	pub fn new() -> Self {
		Self {
			vec: Vec::new(),
			total_weight: 0,
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			vec: Vec::with_capacity(capacity),
			total_weight: 0,
		}
	}

	/// `Vec::reserve`.
	pub fn reserve(&mut self, additional: usize) {
		self.vec.reserve(additional)
	}

	/// `Vec::reserve_exact`.
	pub fn reserve_exact(&mut self, additional: usize) {
		self.vec.reserve_exact(additional)
	}
}

impl<T, U> AsRef<U> for WeightedVec<T>
where
	<WeightedVec<T> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: Debug> Debug for WeightedVec<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("WeightedVec")
			.field("total_weight", &self.total_weight)
			.field("vec", &self.vec)
			.finish()
	}
}

impl<T> Deref for WeightedVec<T> {
	type Target = Vec<WeightedEntry<T>>;

	fn deref(&self) -> &Self::Target {
		&self.vec
	}
}

impl<T> WeightedCollection<T> for WeightedVec<T> {
	fn raffle(&self, partition_weight: usize) -> Option<&WeightedEntry<T>> {
		(&self.vec).get(self.vec.partition_point(|entry| entry.partition_weight < partition_weight))
	}

	fn total_weight(&self) -> usize {
		self.total_weight
	}
}

impl<T> WeightedCollectionMut<T> for WeightedVec<T> {
	fn clear(&mut self) {
		self.total_weight = 0;

		self.vec.clear();
	}

	fn pop(&mut self) -> Option<WeightedItem<T>> {
		let entry = self.vec.pop()?;
		self.total_weight -= entry.weight.get();

		Some(entry.into())
	}

	fn push(&mut self, item: impl Into<WeightedItem<T>>) {
		let item = item.into();

		self.vec.push(WeightedEntry {
			value: item.value,
			weight: item.weight,
			partition_weight: self.total_weight,
		});
	}

	fn raffle_mut(&mut self, partition_weight: usize) -> Option<&mut WeightedEntry<T>> {
		let partition_point = self.vec.partition_point(|entry| entry.partition_weight < partition_weight);

		(&mut self.vec).get_mut(partition_point)
	}
}
