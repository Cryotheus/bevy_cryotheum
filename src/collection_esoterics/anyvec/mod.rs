//! A trait and its impls for different `Vec` types.

#[cfg(feature = "smallvec")]
pub(crate) mod smallvec;

//waiting for https://github.com/bluss/arrayvec/pull/191
//#[cfg(feature = "arrayvec")]
//pub(crate) mod arrayvec;

use std::ops::{Index, IndexMut, RangeBounds};

/// Any `Vec`-like collection used by this crate's collections.
pub trait AnyVec<T>: AsRef<[T]> + Index<usize, Output = T> {
	fn as_slice(&self) -> &[T] {
		self.as_ref()
	}

	fn get(&self, index: usize) -> Option<&T> {
		self.as_slice().get(index)
	}

	fn iter(&self) -> std::slice::Iter<'_, T> {
		self.as_slice().iter()
	}

	fn last(&self) -> Option<&T> {
		self.as_slice().last()
	}

	fn len(&self) -> usize {
		self.as_slice().len()
	}

	fn new() -> Self;

	fn partition_point(&self, pred: impl FnMut(&T) -> bool) -> usize {
		self.as_slice().partition_point(pred)
	}
}

pub trait AnyVecMut<T>: AsMut<[T]> + AnyVec<T> + IndexMut<usize> {
	fn as_slice_mut(&mut self) -> &mut [T] {
		self.as_mut()
	}

	fn clear(&mut self);
	fn drain<R: RangeBounds<usize>>(&mut self, range: R);

	fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.as_slice_mut().get_mut(index)
	}

	fn insert(&mut self, index: usize, element: T);

	fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
		self.as_slice_mut().iter_mut()
	}

	fn pop(&mut self) -> Option<T>;
	fn push(&mut self, value: T);
	fn remove(&mut self, index: usize) -> T;
	fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, f: F);
	fn truncate(&mut self, len: usize);
}

impl<T> AnyVec<T> for Vec<T> {
	fn new() -> Self {
		Self::new()
	}
}

impl<T> AnyVecMut<T> for Vec<T>
where
	Self: AnyVec<T>,
{
	fn clear(&mut self) {
		self.clear()
	}

	/// Unlike [`Vec::drain`], this does not return the drained elements.
	fn drain<R: RangeBounds<usize>>(&mut self, range: R) {
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
