use bevy::prelude::*;
use std::{
	fmt::Debug,
	marker::PhantomData,
	ops::{Deref, DerefMut},
};

/// Wrapper for changing the type ID of an existing type.
/// This can be used in your Bevy game to:
/// - Create multiple global resource instances of "the same type"
/// - Create an event with a primitive type
/// - Create
/// - Add additional context to a resource
/// E.g. a resource like `Res<MyAssets>` can have two additional versions by using
/// `Res<Sign<MyAssets, RandomType>>` and `Res<Sign<MyAssets, AnotherType>>`  
///
/// The `Phantom` generic can be any type you want,
/// but it is preferred if the type is unique and fieldless.
#[derive(Component, Event, Resource)]
pub struct Sign<T, Phantom> {
	phantom: PhantomData<Phantom>,
	inner: T,
}

#[allow(dead_code)]
impl<T, Phantom> Sign<T, Phantom> {
	/// Change the phantom data to a different type, keeping the same data.
	pub fn map<NewPhantom>(self) -> Sign<T, NewPhantom> {
		Sign {
			phantom: PhantomData,
			inner: self.take(),
		}
	}

	/// Creates a wrapper of the value with an additional type to change its type ID.
	/// See `Sign::signed` for a version that accepts a generic type.
	pub fn new(value: T) -> Self {
		Self {
			phantom: PhantomData,
			inner: value,
		}
	}

	/// Same as Sign::new but with a type generic for the Phantom generic.
	pub fn signed<WithPhantom>(value: T) -> Sign<T, WithPhantom> {
		Sign {
			phantom: PhantomData,
			inner: value,
		}
	}

	/// Takes the value of type `T` out of the wrapper, consuming self.
	pub fn take(self) -> T {
		self.inner
	}
}

impl<T: Debug, Phantom> Debug for Sign<T, Phantom> {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		formatter
			.debug_struct("Sign")
			.field("phantom", &self.phantom)
			.field("inner", &self.inner)
			.finish()
	}
}

//I don't know if this is necessary, or if I could just derive Debug
//don't feel like testing, so if you see this please test for me :pleading:
impl<T: Default, Phantom> Default for Sign<T, Phantom> {
	fn default() -> Self {
		Self {
			phantom: PhantomData,
			inner: T::default(),
		}
	}
}

impl<T, Phantom> Deref for Sign<T, Phantom> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T, Phantom> DerefMut for Sign<T, Phantom> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T, Phantom> From<T> for Sign<T, Phantom> {
	fn from(value: T) -> Self {
		Sign::new(value)
	}
}
