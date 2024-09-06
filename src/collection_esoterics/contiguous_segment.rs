use crate::collection_esoterics::anyvec::{AnyVec, AnyVecMut};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};

/// Part of a larger segment with typically from a [`ContiguousSegments`] collection.
/// Same as [`Segment`] but has a length referring to where it is in the collection.
pub struct AlignedSegment<T> {
	alignment: f32,
	segment: Segment<T>,
}

impl<T> AlignedSegment<T> {
	pub fn segment_alignment(&self) -> f32 {
		self.alignment
	}

	/// Same as `self.deref().deref()`.
	pub fn segment_value(&self) -> &T {
		&self.segment.value
	}

	/// Same as `self.deref_mut().deref_mut()`.
	pub fn segment_value_mut(&mut self) -> &mut T {
		&mut self.segment.value
	}
}

impl<T, U> AsRef<U> for AlignedSegment<T>
where
	<Self as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T, U> AsMut<U> for AlignedSegment<T>
where
	<Self as Deref>::Target: AsMut<U>,
{
	fn as_mut(&mut self) -> &mut U {
		self.deref_mut().as_mut()
	}
}

impl<T: Clone> Clone for AlignedSegment<T> {
	fn clone(&self) -> Self {
		Self {
			alignment: self.alignment,
			segment: self.segment.clone(),
		}
	}
}

impl<T: Debug> Debug for AlignedSegment<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("AlignedSegment")
			.field("alignment", &self.alignment)
			.field("segment", &self.segment)
			.finish()
	}
}

impl<T> Deref for AlignedSegment<T> {
	type Target = Segment<T>;

	fn deref(&self) -> &Self::Target {
		&self.segment
	}
}

impl<T> DerefMut for AlignedSegment<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.segment
	}
}

/// Part of a larger segment with typically from a [`ContiguousSegments`] collection.
/// Same as [`AlignedSegment`] but has an index referring to where it is in the collection.
pub struct IndexedSegment<'a, T> {
	index: usize,
	segment: &'a AlignedSegment<T>,
}

impl<'a, T> IndexedSegment<'a, T> {
	pub fn segment_index(&self) -> usize {
		self.index
	}

	pub fn segment_value(&self) -> &T {
		&self.segment.segment.value
	}
}

impl<'a, T, U> AsRef<U> for IndexedSegment<'a, T>
where
	<Self as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<'a, T: Debug> Debug for IndexedSegment<'a, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("IndexedSegment")
			.field("index", &self.index)
			.field("segment", &self.segment)
			.finish()
	}
}

impl<'a, T> Deref for IndexedSegment<'a, T> {
	type Target = &'a AlignedSegment<T>;

	fn deref(&self) -> &Self::Target {
		&self.segment
	}
}

impl<'a, T: Clone> From<IndexedSegment<'a, T>> for AlignedSegment<T> {
	fn from(value: IndexedSegment<T>) -> Self {
		value.segment.clone()
	}
}

/// Mutable version of [`IndexedSegment`].
pub struct IndexedSegmentMut<'a, T> {
	index: usize,
	segment: &'a mut AlignedSegment<T>,
}

impl<'a, T> IndexedSegmentMut<'a, T> {
	pub fn index(&self) -> usize {
		self.index
	}

	pub fn segment_value(&self) -> &T {
		&self.segment.segment.value
	}

	pub fn segment_value_mut(&mut self) -> &mut T {
		&mut self.segment.segment.value
	}
}

impl<'a, T, U> AsRef<U> for IndexedSegmentMut<'a, T>
where
	<Self as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<'a, T, U> AsMut<U> for IndexedSegmentMut<'a, T>
where
	<Self as Deref>::Target: AsMut<U>,
{
	fn as_mut(&mut self) -> &mut U {
		self.deref_mut().as_mut()
	}
}

impl<'a, T: Debug> Debug for IndexedSegmentMut<'a, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("IndexedSegmentMut")
			.field("index", &self.index)
			.field("segment", &self.segment)
			.finish()
	}
}

impl<'a, T> Deref for IndexedSegmentMut<'a, T> {
	type Target = &'a mut AlignedSegment<T>;

	fn deref(&self) -> &Self::Target {
		&self.segment
	}
}

impl<'a, T> DerefMut for IndexedSegmentMut<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.segment
	}
}

impl<'a, T: Clone> From<IndexedSegmentMut<'a, T>> for AlignedSegment<T> {
	fn from(value: IndexedSegmentMut<T>) -> Self {
		value.segment.clone()
	}
}

/// A value with a length utilized by a [`ContiguousSegments`] collection.
pub struct Segment<T> {
	length: f32,
	value: T,
}

impl<T> Segment<T> {
	pub fn into_inner(self) -> T {
		self.value
	}

	pub fn new(value: T, length: f32) -> Self {
		Self { length, value }
	}

	pub fn segment_length(&self) -> f32 {
		self.length
	}

	fn set_segment_length(&mut self, length: f32) {
		self.length = length;
	}

	/// Same as `deref`.
	pub fn segment_value(&self) -> &T {
		&self.value
	}

	/// Same as `deref_mut`.
	pub fn segment_value_mut(&mut self) -> &mut T {
		&mut self.value
	}
}

impl<T, U> AsRef<U> for Segment<T>
where
	<Self as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T, U> AsMut<U> for Segment<T>
where
	<Self as Deref>::Target: AsMut<U>,
{
	fn as_mut(&mut self) -> &mut U {
		self.deref_mut().as_mut()
	}
}

impl<T: Clone> Clone for Segment<T> {
	fn clone(&self) -> Self {
		Self {
			length: self.length,
			value: self.value.clone(),
		}
	}
}

impl<T: Debug> Debug for Segment<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Segment")
			.field("length", &self.length)
			.field("value", &self.value)
			.finish()
	}
}

impl<T> Deref for Segment<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.value
	}
}

impl<T> DerefMut for Segment<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.value
	}
}

/// Contiguous sequence of [`AlignedSegment`]s.
pub struct ContiguousSegments<T, C = Vec<AlignedSegment<T>>>
where
	C: AnyVecMut<AlignedSegment<T>>,
{
	segments: C,
	total_length: f32,
	phantom: PhantomData<T>,
}

impl<T, C: AnyVecMut<AlignedSegment<T>>> ContiguousSegments<T, C> {
	pub fn new() -> Self {
		Self {
			segments: C::new(),
			total_length: 0.,
			phantom: PhantomData,
		}
	}

	/// Removes segments with invalid lengths and calls `realign`.
	pub fn clean(&mut self) {
		self.segments.retain_mut(
			|AlignedSegment {
			     segment: Segment { length, .. },
			     ..
			 }| length.is_normal() && length.is_sign_positive(),
		);

		self.realign();
	}

	/// Returns the amount of segments which make up the whole chain segment.
	pub fn count(&self) -> usize {
		self.segments.len()
	}

	pub fn from_segment(segment: Segment<T>) -> Self {
		let mut contig = Self {
			segments: C::new(),
			total_length: segment.length,
			phantom: PhantomData,
		};

		contig.segments.push(AlignedSegment { alignment: 0.0, segment });

		contig
	}

	pub fn from_segments(segments_iter: impl Iterator<Item = Segment<T>>) -> Self {
		let mut contig = Self::new();

		for segment in segments_iter {
			contig.push(segment);
		}

		contig
	}

	pub fn get(&self, index: usize) -> Option<IndexedSegment<T>> {
		Some(IndexedSegment {
			index,
			segment: self.segments.get(index)?,
		})
	}

	/// Get the length at which the segment of the specified index starts in the whole.
	pub fn get_alignment(&self, index: usize) -> Option<f32> {
		self.segments.get(index).map(|AlignedSegment { alignment, .. }| *alignment)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<IndexedSegmentMut<T>> {
		Some(IndexedSegmentMut {
			index,
			segment: self.segments.get_mut(index)?,
		})
	}

	/// Gets the segment at the specified length along the whole.
	pub fn get_at(&self, length: f32) -> Option<IndexedSegment<T>> {
		let parition_point = self.partition_point(length);

		self.get(parition_point)
	}

	/// Get the alignment of the segment at the specified length along the whole.
	pub fn get_alignment_at(&self, length: f32) -> Option<f32> {
		let parition_point = self.partition_point(length);

		self.segments.get(parition_point).map(|AlignedSegment { alignment, .. }| *alignment)
	}

	/// Get an index-alignment tuple of the segment at the specified length along the whole.
	pub fn get_ia_at(&self, length: f32) -> Option<(usize, f32)> {
		let parition_point = self.partition_point(length);

		Some((parition_point, self.segments.get(parition_point)?.alignment))
	}

	/// Get the index of the segment at the specified length along the whole.
	/// Same as `partition_point` but returns `None` if the index is out of range.
	pub fn get_index_at(&self, length: f32) -> Option<usize> {
		let parition_point = self.partition_point(length);

		if parition_point < self.count() {
			Some(parition_point)
		} else {
			None
		}
	}

	/// Gets the segment at the specified length along the whole.
	pub fn get_mut_at(&mut self, length: f32) -> Option<IndexedSegmentMut<T>> {
		let parition_point = self.partition_point(length);

		self.get_mut(parition_point)
	}

	/// # Panics
	/// If the index is out of bounds.
	pub fn get_length(&mut self, index: usize) -> f32 {
		self.segments[index].length
	}

	/// # Panics
	/// If `index > len`.
	pub fn insert(&mut self, index: usize, segment: Segment<T>) {
		self.segments.insert(index, AlignedSegment { alignment: 0.0, segment });
		self.realign_from(index);
	}

	/// Inserts a segment before the segment at the specified length.
	/// Returns the index of where the segment was inserted.
	pub fn insert_at(&mut self, length: f32, segment: Segment<T>) -> usize {
		let parition_point = self.partition_point(length);

		if self.count() == parition_point {
			self.push(segment);
		} else {
			self.insert(parition_point, segment);
		}

		parition_point
	}

	/// Combines neighboring segments of equal value.
	pub fn merge(&mut self)
	where
		T: PartialEq,
	{
		//merge function could be improved
		let count = self.count();

		if count < 2 {
			return;
		}

		let mut remove = Vec::new();
		let mut previous_index = 0;
		let mut previous_ref = &self.segments[previous_index];

		for index in 1..count {
			if previous_ref.eq(&self.segments[index]) {
				remove.push(index);
			} else {
				self.segments[previous_index].length = self.segments[index].alignment - previous_ref.alignment;
				previous_ref = &self.segments[index];
				previous_index = index;
			}
		}

		for index in remove.iter().rev() {
			self.segments.remove(*index);
		}

		self.realign();
	}

	pub fn partition_point(&self, length: f32) -> usize {
		self.segments.partition_point(
			|AlignedSegment {
			     alignment,
			     segment: Segment { length: seg_length, .. },
			 }| (*alignment + *seg_length) < length,
		)
	}

	pub fn pop(&mut self) -> Option<Segment<T>> {
		let popped = self.segments.pop()?;
		self.total_length = popped.alignment;

		Some(popped.segment)
	}

	pub fn push(&mut self, segment: Segment<T>) {
		let segment_length = segment.length;

		self.segments.push(AlignedSegment {
			segment,
			alignment: self.total_length,
		});

		self.total_length += segment_length;
	}

	/// Realign all segments in the chain segment.
	pub fn realign(&mut self) {
		self.total_length = 0f32;

		for part in self.segments.iter_mut() {
			part.alignment = self.total_length;
			self.total_length += part.length;
		}
	}

	/// Realigns all segments from the `start` index, inclusive.
	pub fn realign_from(&mut self, start: usize) {
		if start == 0 {
			self.realign();

			return;
		} else if start >= self.count() {
			return;
		}

		let previous_part = &self.segments[start - 1];
		let mut running_alignment = previous_part.alignment + previous_part.length;

		//realign everything after the part we set
		for AlignedSegment {
			alignment,
			segment: Segment { length, .. },
		} in &mut self.segments.as_slice_mut()[start..]
		{
			*alignment = running_alignment;
			running_alignment += *length;
		}

		self.total_length = running_alignment;
	}

	/// # Panics
	/// If the index is out of bounds.
	pub fn set_length(&mut self, index: usize, length: f32) {
		self.segments[index].set_segment_length(length);

		if self.count() == 1 {
			self.total_length = length;

			return;
		}

		self.realign_from(index + 1);
	}

	/// The start or end of the range must overlap an existing segment.
	pub fn set_range(&mut self, range: impl RangeBounds<f32>, value: T) -> Option<usize>
	where
		T: Clone,
	{
		use Bound::*;

		match [range.start_bound().cloned(), range.end_bound().cloned()] {
			//make the whole thing a single segment
			[Unbounded, Unbounded] => self.set_whole(value),

			[Excluded(start_bound) | Included(start_bound), Unbounded] => {
				let original_total_length = self.total_length;
				let mut start_segment = self.get_mut_at(start_bound)?;

				if start_segment.alignment == start_bound {
					*start_segment.segment_value_mut() = value;
					start_segment.length = original_total_length - start_segment.alignment;

					Some(start_segment.index)
				} else {
					start_segment.length = start_bound - start_segment.alignment;
					self.total_length = start_segment.alignment + start_segment.length;

					self.push(Segment {
						length: original_total_length - self.total_length,
						value,
					});

					Some(self.count() - 1)
				}
			}

			[Excluded(start_bound) | Included(start_bound), Excluded(end_bound) | Included(end_bound)] => {
				match [self.get_ia_at(start_bound), self.get_ia_at(end_bound)] {
					[Some((start_index, start_alignment)), Some((end_index, end_alignment))] => {
						let length = end_bound - start_bound;

						//if we're within a single segment, we can take advantage of that
						if start_index == end_index {
							return if start_alignment == start_bound {
								//two-part split
								let mut start_segment = self.get_mut(start_index).unwrap();
								start_segment.alignment += length;
								start_segment.length -= length;

								self.segments.insert(
									start_index,
									AlignedSegment {
										segment: Segment { length, value },
										alignment: start_alignment,
									},
								);

								Some(start_index)
							} else {
								//three-part split
								let low_segment = self.segments.get_mut(start_index).unwrap();
								let cloned_value = low_segment.segment_value().clone();
								let original_length = low_segment.length;
								low_segment.length = start_bound - low_segment.alignment;

								self.segments.insert(
									start_index + 1,
									AlignedSegment {
										segment: Segment { length, value },
										alignment: start_bound,
									},
								);

								self.segments.insert(
									start_index + 2,
									AlignedSegment {
										segment: Segment {
											length: start_alignment + original_length - end_bound,
											value: cloned_value,
										},
										alignment: start_bound + length,
									},
								);

								Some(start_index + 1)
							};
						}

						if start_alignment == start_bound {
							//delete the elements in between
							self.segments.drain((start_index + 1)..end_index);

							//start segment, turned into the range segment
							let segment = &mut self.segments[start_index];
							segment.length = end_bound - start_bound;
							segment.value = value;

							//end segment
							let segment = &mut self.segments[start_index + 1];
							segment.alignment = end_bound;
							segment.length -= end_bound - segment.alignment;

							Some(start_index)
						} else {
							match end_index - start_index {
								//already checked above with start_alignment == end_index
								0 => unreachable!(),

								//no gap
								1 => {
									//start segment
									let segment = &mut self.segments[start_index];
									segment.length = start_bound - segment.alignment;

									//end segment
									let segment = &mut self.segments[start_index + 1];
									segment.alignment = end_bound;
									segment.length -= end_bound - segment.alignment;

									//new segment covering the range
									self.segments.insert(
										start_index + 1,
										AlignedSegment {
											segment: Segment { length, value },
											alignment: start_bound,
										},
									);
								}

								//1 or more entry in gap
								gap => {
									if gap > 2 {
										//more than 1 entry in gap
										//get rid of them
										self.segments.drain((start_index + 2)..(start_index + gap));
									}

									//start segment
									let segment = &mut self.segments[start_index];
									segment.length = start_bound - segment.alignment;

									//center segment - turned range segment
									let segment = &mut self.segments[start_index + 1];
									segment.alignment = start_bound;
									segment.length = length;
									segment.value = value;

									//end segment
									let segment = &mut self.segments[start_index + 2];
									segment.alignment = end_bound;
									segment.length -= end_bound - segment.alignment;
								}
							}

							Some(start_index + 1)
						}
					}

					//defined start
					//off-segment end
					[Some((start_index, start_alignment)), None] => {
						self.total_length = end_bound;

						if start_alignment == start_bound {
							self.segments.truncate(start_index + 1);

							let segment = &mut self.segments[start_index];
							segment.value = value;
							segment.length = end_bound - start_bound;

							Some(start_index)
						} else {
							let segment = &mut self.segments[start_index];
							segment.length = start_bound - segment.alignment;

							if let Some(next_segment) = self.segments.get_mut(start_index + 1) {
								next_segment.alignment = start_bound;
								next_segment.length = end_bound - start_bound;
								next_segment.value = value;

								self.segments.truncate(start_index + 2);
							} else {
								self.push(Segment {
									length: end_bound - start_bound,
									value,
								});
							}

							Some(start_index + 1)
						}
					}

					//this should be impossible since the partition function is used
					//[None, Some(_)] => self.set_range((Bound::Unbounded, end_bound_enum), value),
					[None, Some(_)] => unreachable!(),

					//this should be impossible since the partition function is used
					/*[None, None] => {
						self.segments.clear();
						self.total_length = end_bound - start_bound;

						self.segments.push(AlignedSegment {
							segment: Segment {
								length: self.total_length,
								value,
							},
							alignment: 0.,
						});

						Some(0)
					} // */
					[None, None] => unreachable!(),
				}
			}

			bounds => todo!("add method to handle {:?}", bounds),
		}
	}

	/// Sets the entirety of the `ContiguousSegment` to a single segment with matching length.
	pub fn set_whole(&mut self, value: T) -> Option<usize> {
		let count = self.count();

		//can't set the whole if there are no segments to begin with
		if count == 0 {
			return None;
		}

		//remove other entries
		if count > 1 {
			self.segments.truncate(1);
		}

		let segment = &mut self.segments[0];
		segment.length = self.total_length;
		segment.value = value;

		Some(0)
	}

	/// Splits a segment into two at the specified length along the whole.
	pub fn split_at(&mut self, length: f32) -> Option<[IndexedSegment<T>; 2]>
	where
		T: Clone,
	{
		let low_index = self.get_index_at(length)?;
		let low_segment = self.segments.get_mut(low_index).unwrap();
		let high_index = low_index + 1;
		let high_length = low_segment.length + low_segment.alignment - length;
		low_segment.length = length - low_segment.alignment;

		let value = low_segment.segment_value().clone();

		self.segments.insert(
			high_index,
			AlignedSegment {
				segment: Segment { length: high_length, value },
				alignment: length,
			},
		);

		Some([
			IndexedSegment {
				index: low_index,
				segment: &self.segments[low_index],
			},
			IndexedSegment {
				index: high_index,
				segment: &self.segments[high_index],
			},
		])
	}

	pub fn total_length(&self) -> f32 {
		self.total_length
	}

	pub fn truncate(&mut self, len: usize) {
		self.segments.truncate(len);

		if let Some(last) = self.segments.last() {
			self.total_length = last.alignment + last.length;
		}
	}

	/// Truncate the contiguous segment making its `total_length` at most `length`.
	/// The segment at the specified `length` will be shortened, and following segments will be dropped.
	pub fn truncate_at(&mut self, length: f32) {
		if length >= self.total_length {
			return;
		}

		let parition_point = self.partition_point(length);
		let Some(segment) = self.segments.get_mut(parition_point) else {
			return;
		};

		segment.length = length - segment.alignment;
		self.total_length = length;

		self.truncate(parition_point + 1);
	}
}

impl<T, C: AnyVecMut<AlignedSegment<T>>> AsRef<[AlignedSegment<T>]> for ContiguousSegments<T, C> {
	fn as_ref(&self) -> &[AlignedSegment<T>] {
		self.segments.as_slice()
	}
}

impl<T, C: AnyVecMut<AlignedSegment<T>> + Clone> Clone for ContiguousSegments<T, C> {
	fn clone(&self) -> Self {
		Self {
			segments: self.segments.clone(),
			total_length: self.total_length,
			phantom: PhantomData,
		}
	}
}

impl<T: Debug, C: AnyVecMut<AlignedSegment<T>>> Debug for ContiguousSegments<T, C> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ContiguousSegments").field("segments", &self.segments.as_slice()).finish()
	}
}

impl<T, C: AnyVecMut<AlignedSegment<T>>> From<Segment<T>> for ContiguousSegments<T, C> {
	fn from(segment: Segment<T>) -> Self {
		Self::from_segment(segment)
	}
}
