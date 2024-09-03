use crate::utils::IsAlphaNumeric;
use anyhow::{anyhow, bail};
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::str::FromStr;
use std::sync::{Arc, Weak};

pub struct ArcRegistry<T: RegistryItem>(Registry<Arc<T>>);

impl<T: RegistryItem> ArcRegistry<T> {
	pub fn get_arc(&self, registry_id: impl AsRef<RegistryId>) -> Option<Arc<T>> {
		self.0.get(registry_id).map(|arc| Arc::clone(arc))
	}

	pub fn get_ref(&self, registry_id: impl AsRef<RegistryId>) -> Option<&T> {
		match self.0.get(registry_id) {
			None => None,
			Some(arc) => Some(arc.as_ref()),
		}
	}

	pub fn get_weak(&self, registry_id: impl AsRef<RegistryId>) -> Option<Weak<T>> {
		self.0.get(registry_id).map(|arc| Arc::downgrade(arc))
	}

	/// Inserts a new RegistryItem into the Registry.
	pub fn insert(&mut self, registry_id: impl Into<RegistryId>, item: T) -> Result<usize, RegistryError> {
		self.0.insert(registry_id, Arc::new(item))
	}

	pub fn insert_all(&mut self, registry_ids: impl IntoIterator<Item = (impl Into<RegistryId>, T)>) -> Result<(), RegistryErrors> {
		self.0.insert_all(registry_ids.into_iter().map(|(id, item)| (id, Arc::new(item))))
	}

	pub fn new() -> Self {
		Self(Registry::new())
	}
}

impl<T: RegistryItem, U> AsRef<U> for ArcRegistry<T>
where
	<ArcRegistry<T> as Deref>::Target: AsRef<U>,
{
	fn as_ref(&self) -> &U {
		self.deref().as_ref()
	}
}

impl<T: RegistryItem> Default for ArcRegistry<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: RegistryItem> Deref for ArcRegistry<T> {
	type Target = Registry<Arc<T>>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T: RegistryItem> DerefMut for ArcRegistry<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

/// A collection of `T` that contains both keys and indices for each registered item.
#[derive(Debug, Resource)]
pub struct Registry<T: RegistryItem> {
	/// Maps to the index of the item in the Vec.
	ids: HashMap<RegistryId, usize>,

	/// The registered items.
	items: Vec<(RegistryId, T)>,
}

impl<T: RegistryItem> Registry<T> {
	/// Clears all items and ids from the registry.
	pub fn clear(&mut self) {
		self.ids.clear();
		self.ids.shrink_to(4);
		self.items.clear();
		self.items.shrink_to(4);
	}

	/// Returns a reference to the registry item with the associated id.
	pub fn get(&self, registry_id: impl AsRef<RegistryId>) -> Option<&T> {
		match self.items.get(*self.ids.get(registry_id.as_ref())?) {
			None => None,
			Some((_, item)) => Some(&item),
		}
	}

	/// Returns a mutable reference to the registry item with the associated id.
	pub fn get_mut(&mut self, registry_id: impl AsRef<RegistryId>) -> Option<&mut T> {
		let index = *self.ids.get(registry_id.as_ref())?;

		if index >= self.items.len() {
			None
		} else {
			Some(&mut self.items[index].1)
		}
	}

	/// Returns the `RegistryId` of the item at the provided index.
	pub fn id_of(&self, index: usize) -> Option<&RegistryId> {
		if index >= self.items.len() {
			None
		} else {
			Some(&self.items[index].0)
		}
	}

	pub fn ids(&self) -> &HashMap<RegistryId, usize> {
		&self.ids
	}

	pub fn items(&self) -> &Vec<(RegistryId, T)> {
		&self.items
	}

	/// Returns the index in the registry at which the associated item is located.
	pub fn index_of(&self, registry_id: impl AsRef<RegistryId>) -> Option<usize> {
		self.ids.get(registry_id.as_ref()).map(|index| *index)
	}

	/// Inserts a new RegistryItem into the Registry.
	pub fn insert(&mut self, registry_id: impl Into<RegistryId>, item: T) -> Result<usize, RegistryError> {
		let registry_id = registry_id.into();

		if self.ids.contains_key(&registry_id) {
			return Err(RegistryError::DuplicateId(registry_id));
		}

		let index = self.items.len();

		self.ids.insert(registry_id.clone(), index);
		self.items.push((registry_id, item));

		Ok(index)
	}

	pub fn insert_all(&mut self, registry_ids: impl IntoIterator<Item = (impl Into<RegistryId>, T)>) -> Result<(), RegistryErrors> {
		let iter = registry_ids.into_iter();
		let (lower_hint, upper_hint_opt) = iter.size_hint();
		let mut errors = Vec::with_capacity(upper_hint_opt.map(|upper_hint| upper_hint - lower_hint).unwrap_or(2));

		for (registry_id, item) in iter {
			errors.push(self.insert(registry_id, item).err());
		}

		if errors.is_empty() {
			Ok(())
		} else {
			Err(RegistryErrors::OptionalErrors(errors))
		}
	}

	pub fn new() -> Self {
		Self {
			ids: HashMap::new(),
			items: Vec::new(),
		}
	}
}

impl<T: RegistryItem> Default for Registry<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: RegistryItem> Index<usize> for Registry<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.items[index].1
	}
}

impl<T: RegistryItem> Index<&RegistryId> for Registry<T> {
	type Output = T;

	fn index(&self, index: &RegistryId) -> &Self::Output {
		&self.items[*self.ids.get(index).expect("failed to index Registry")].1
	}
}

impl<T: RegistryItem> IndexMut<usize> for Registry<T> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.items[index].1
	}
}

impl<T: RegistryItem> IndexMut<&RegistryId> for Registry<T> {
	fn index_mut(&mut self, index: &RegistryId) -> &mut Self::Output {
		&mut self.items[*self.ids.get(index).expect("failed to index Registry")].1
	}
}

impl<T: RegistryItem> IntoIterator for Registry<T> {
	type Item = (RegistryId, T);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.items.into_iter()
	}
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
	#[error("RegistryId {} is already registered", .0)]
	DuplicateId(RegistryId),
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryErrors {
	#[error("Multiple registry errors")]
	Errors(Vec<RegistryError>),

	#[error("Multiple registry errors aligned with their input")]
	OptionalErrors(Vec<Option<RegistryError>>),
}

#[derive(Clone, Debug, Eq)]
pub struct RegistryId {
	colon: usize,
	string: String,
}

/// Represents a unique identifier for a registered data type.
impl RegistryId {
	pub fn id(&self) -> &str {
		&self.string
	}

	pub fn name(&self) -> &str {
		&self.string[(self.colon + 1)..]
	}

	pub fn new(source: String, name: String) -> Self {
		Self {
			colon: source.len(),
			string: format!("{source}:{name}"),
		}
	}

	pub fn source(&self) -> &str {
		&self.string[..self.colon]
	}
}

impl Display for RegistryId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.string)
	}
}

impl FromStr for RegistryId {
	type Err = anyhow::Error;

	fn from_str(str: &str) -> Result<Self, Self::Err> {
		if !str.is_ascii() {
			bail!("RegistryId must be ascii");
		}

		if str != &str.to_lowercase() {
			bail!("RegistryId must be all lowercase ASCII");
		}

		let mut spliterator = str.split(":");

		fn starts_with_lowercase_letter(str: &str) -> bool {
			let byte = str.as_bytes()[0];

			byte >= b'a' && byte <= b'z'
		}

		match (spliterator.next(), spliterator.next(), spliterator.next()) {
			(Some(source), Some(name), None) => {
				if !(starts_with_lowercase_letter(source) && starts_with_lowercase_letter(name)) {
					bail!("RegistryId input string must start with a lower-case ASCII letter");
				}

				if !(source.is_alpha_numeric() && name.is_alpha_numeric()) {
					bail!("RegistryId input string must start with a lower-case ASCII letter");
				}

				Ok(Self::new(source.to_string(), name.to_string()))
			}

			_ => Err(anyhow!("RegistryId expected format \"source:name\" (which was not provided)")),
		}
	}
}

impl From<&str> for RegistryId {
	fn from(value: &str) -> Self {
		Self::from_str(value).unwrap()
	}
}

impl From<(&str, &str)> for RegistryId {
	fn from(value: (&str, &str)) -> Self {
		Self::new(value.0.to_owned(), value.1.to_owned())
	}
}

impl From<[&str; 2]> for RegistryId {
	fn from(value: [&str; 2]) -> Self {
		Self::new(value[0].to_owned(), value[1].to_owned())
	}
}

impl From<String> for RegistryId {
	fn from(value: String) -> Self {
		Self::from_str(&value).unwrap()
	}
}

impl Hash for RegistryId {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.string.hash(state);
	}
}

impl Ord for RegistryId {
	fn cmp(&self, other: &Self) -> Ordering {
		self.string.cmp(&other.string)
	}
}

impl PartialEq for RegistryId {
	fn eq(&self, other: &Self) -> bool {
		self.string == other.string
	}
}

impl PartialOrd for RegistryId {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.string.partial_cmp(&other.string)
	}
}

/// Implemented by data types that can be added to a `Registry<T>`.
pub trait RegistryItem: Debug {}

impl<T: Deref + Debug> RegistryItem for T where <T as Deref>::Target: RegistryItem {}
