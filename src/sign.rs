//! See [`Sign`].

use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use bevy::ecs::component::{ComponentHooks, StorageType};
use bevy::prelude::{Component, Event, Resource};

/// Wrapper for changing the type ID of an existing type.
/// This can be used in your Bevy game to:
/// - Create multiple global resource instances of "the same type"
/// - Create an event with a primitive type
/// - Add additional context to a resource
/// E.g. a resource like `Res<MyAssets>` can have two additional versions by using
/// `Res<Sign<MyAssets, RandomType>>` and `Res<Sign<MyAssets, AnotherType>>`
///
/// The `Phantom` generic can be any type you want,
/// but it is preferred if the type is unique and zero-sized.
pub struct Sign<T, Phantom: ?Sized> {
	inner: T,
	phantom: PhantomData<Phantom>,
}

#[allow(dead_code)]
impl<T, Phantom: ?Sized> Sign<T, Phantom> {
	/// Creates a `Sign<T>` for any value that can be converted into `T`.
	pub fn from(value: impl Into<T>) -> Sign<T, Phantom> {
		Sign {
			inner: value.into(),
			phantom: PhantomData,
		}
	}

	/// Change the phantom data to a different type, keeping the same data.
	pub fn map_phantom<NewPhantom>(self) -> Sign<T, NewPhantom> {
		Sign {
			phantom: PhantomData,
			inner: self.inner,
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

	/// Returns a copy of the phantom data.
	pub fn phantom(&self) -> PhantomData<Phantom> {
		self.phantom
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

	pub fn take_phantom(self) -> PhantomData<Phantom> {
		self.phantom
	}
}

//unitraits!
impl<T, Phantom: ?Sized> Deref for Sign<T, Phantom> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T, Phantom: ?Sized> DerefMut for Sign<T, Phantom> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T, Phantom: ?Sized> From<T> for Sign<T, Phantom> {
	fn from(value: T) -> Self {
		Sign::new(value)
	}
}

//conditional traits
impl<T, Phantom: Default + ?Sized> Sign<T, Phantom> {
	/// Calls `default()` for the `Phantom`.
	pub fn default_phantom_instance(&self) -> Phantom {
		Phantom::default()
	}
}

impl<T, U, Phantom: ?Sized> AsMut<U> for Sign<T, Phantom>
where
	<Sign<T, Phantom> as Deref>::Target: AsMut<U>,
{
	fn as_mut(&mut self) -> &mut U {
		self.deref_mut().as_mut()
	}
}

impl<T, U, Phantom: ?Sized> AsRef<U> for Sign<T, Phantom>
where
	<Sign<T, Phantom> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: Clone, Phantom: ?Sized> Clone for Sign<T, Phantom> {
	fn clone(&self) -> Self {
		Self {
			phantom: PhantomData,
			inner: T::clone(&self.inner),
		}
	}
}

impl<T: Component, Phantom: Send + Sync + 'static + ?Sized> Component for Sign<T, Phantom> {
	const STORAGE_TYPE: StorageType = <T as Component>::STORAGE_TYPE;

	fn register_component_hooks(hooks: &mut ComponentHooks) {
		<T as Component>::register_component_hooks(hooks)
	}
}

impl<T: Clone + Copy, Phantom: ?Sized> Copy for Sign<T, Phantom> {}

impl<T: Debug, Phantom: ?Sized> Debug for Sign<T, Phantom> {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
		formatter
			.debug_struct("Sign")
			.field("inner", &self.inner)
			.field("phantom", &self.phantom)
			.finish()
	}
}

impl<T: Default, Phantom: ?Sized> Default for Sign<T, Phantom> {
	fn default() -> Self {
		Self {
			phantom: PhantomData,
			inner: T::default(),
		}
	}
}

impl<T: Display, Phantom: ?Sized> Display for Sign<T, Phantom> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		T::fmt(&self.inner, f)
	}
}

impl<T: Event, Phantom: Send + Sync + 'static + ?Sized> Event for Sign<T, Phantom> {}

impl<T: Resource, Phantom: Send + Sync + 'static + ?Sized> Resource for Sign<T, Phantom> {}
