use std::ops::RangeBounds;
use arrayvec::ArrayVec;
use crate::collection_esoterics::vec_trait::{CeVec, CeVecMut};

impl<T, const CAP: usize> CeVec<T> for ArrayVec<T, CAP> {
	fn new() -> Self {
		Self::new()
	}
}

impl<T, const CAP: usize> CeVecMut<T> for ArrayVec<T, CAP>
where
	Self: CeVec<T>,
{
	fn clear(&mut self) {
		self.clear()
	}

	/// Unlike [`Vec::drain`], this does not return the drained elements.
	fn drain<B: RangeBounds<usize>>(&mut self, range: B) {
		self.drain(range);
	}

	fn insert(&mut self, index: usize, element: T) {
		self.insert(index, element)
	}

	fn pop(&mut self) -> Option<T> {
		self.pop()
	}

	fn push(&mut self, value: T) {
		self.push(value)
	}

	fn remove(&mut self, index: usize) -> T {
		self.remove(index)
	}

	fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, f: F) {
		self.retain_mut(f)
	}

	fn truncate(&mut self, len: usize) {
		self.truncate(len)
	}
}
