//! Provides the [`MaterialToml`] data type for easily loading materials without requiring a recompile.

use bevy::asset::AssetServer;
use bevy::color::{Color, LinearRgba};
use bevy::log::error;
use bevy::math::{Mat2, Vec2};
use bevy::pbr::{ExtendedMaterial, MaterialExtension, ParallaxMappingMethod, StandardMaterial};
use bevy::prelude::default;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::render::texture::{ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Implements `load_material` and `load_material_base` functions for [`AssetServer`]
/// to make loading materials from their [`MaterialToml`] easier.
pub trait LoadStandardMaterial {
	/// Convenience function for loading a [`MaterialToml`] and immediately loading
	/// a [`StandardMaterial`] from it. fn load_material(&self, path: impl Into<PathBuf>) -> StandardMaterial;
	fn load_material(&self, path: impl Into<PathBuf>) -> StandardMaterial;

	/// Convenience function for loading a [`MaterialToml`] and immediately loading
	/// a [`ExtendedMaterial`] with [`StandardMaterial`] as the base.
	fn load_material_base<E: MaterialExtension>(&self, path: impl Into<PathBuf>, extension: E) -> ExtendedMaterial<StandardMaterial, E> {
		ExtendedMaterial {
			base: self.load_material(path),
			extension,
		}
	}
}

impl LoadStandardMaterial for AssetServer {
	fn load_material(&self, path: impl Into<PathBuf>) -> StandardMaterial {
		let path = path.into();

		MaterialToml::new(&path)
			.unwrap_or_else(|_| {
				error!(
					"<AssetServer as LoadStandardMaterial>::load_material failed to load a MaterialToml at path {path:?} CANON: {:?}",
					path.canonicalize()
				);

				MaterialToml::default()
			})
			.load(&self)
	}
}

/// Configuration for loading materials with multiple textures and custom settings.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MaterialToml {
	/// Loads `ao`.
	pub ao: Option<bool>,

	/// Loads `clearcoat`.
	pub clearcoat: Option<f32>,

	/// Loads `clearcoat_normal` or `normal`.
	/// No option for flipping the Y axis is available with this normal map.
	pub clearcoat_normal: Option<MaterialTomlClearcoatMode>,

	/// Loads `clearcoat_rough`.
	pub clearcoat_rough: Option<f32>,

	/// Base color for the material in LinearRGB or LinearRGBA color space.
	/// Does not load any textures.
	pub color: Option<(f32, f32, f32, Option<f32>)>,

	/// Parallax mapping.
	/// Loads `height`.
	pub depth: Option<f32>,

	/// Set to `Some(false)` or `None` to use nearest neighbor in depth-map sampling.
	/// Can be a major deficit to performance.
	/// Does not load any textures.
	pub depth_hq: Option<bool>,

	/// Direct binding to [`StandardMaterial`] `max_parallax_layer_count`.
	/// Does not load any textures.
	pub depth_layers: Option<f32>,

	/// The rendering method for parallax mapping.
	/// Does not load any textures.
	pub depth_method: Option<u32>,

	/// Emissive lighting.
	/// Loads `emissive` texture.
	pub emissive: Option<bool>,

	/// Emissive lighting color in LinearRGB or LinearRGBA color space.
	/// Loads `emissive` texture.
	pub emissive_color: Option<(f32, f32, f32)>,

	/// How much exposure impacts the emissive light of this material.
	/// Defaults to 1 which is best for realistic-light emitting materials.
	/// Using 0 means no exposure adjustments, and will guarantee the lighting to always be bright.
	/// Does not load any textures.
	pub emissive_exposure: Option<f32>,

	/// File extension for all texture files.
	pub extension: Option<String>,

	/// Loads `combo_0rm`.
	pub metal: Option<f32>,

	/// Loads `combo_0rm`.
	pub rough: Option<f32>,

	/// Loads `normal`, and the enum decides if we should flip the y axis.
	pub normal: Option<MaterialTomlNormalsYDir>,

	/// Does not load any textures.
	pub reflectance: Option<f32>,

	/// Specular TRANSMISSION not reflection (for glass-like materials) so use rough instead.
	/// Loads `specular_trans`.
	pub specular_trans: Option<f32>,

	/// Settings to `Some(true)` enables texture tiling.
	/// Does not load any textures.
	pub tile: Option<bool>, //

	pub uv_offset: Option<Vec2>,

	pub uv_scale: Option<Vec2>,

	/// The path where the material toml was loaded from, or should be saved to.
	#[serde(skip)]
	pub path: Option<PathBuf>,
}

impl MaterialToml {
	pub fn toml_path(&self) -> Option<PathBuf> {
		let path = self.path.clone()?;

		Some(if path.is_relative() {
			PathBuf::from("assets").join(&path)
		} else {
			path.clone()
		})
	}

	/// # Panics
	/// If the path field is `None` or the path has no parent.
	pub fn dir(&self) -> Option<&Path> {
		let path_ref: &Path = self.path.as_ref()?.as_ref();

		path_ref.parent()
	}

	/// An example material toml with a bunch of fields set to arbitrary values.
	pub fn example() -> Self {
		Self {
			ao: Some(true),
			clearcoat: Some(1.0),
			clearcoat_normal: Some(MaterialTomlClearcoatMode::CustomNormal),
			clearcoat_rough: Some(1.0),
			color: Some((0.25, 0.8125, 0.125, Some(0.625))),
			depth: Some(0.0025),
			depth_hq: Some(false),
			depth_layers: Some(16.),
			depth_method: Some(8),
			emissive: Some(true),
			emissive_color: Some((1., 1., 1.)),
			emissive_exposure: Some(1.0),
			extension: None,
			metal: Some(1.),
			reflectance: Some(0.5),
			rough: Some(1.),
			normal: Some(MaterialTomlNormalsYDir::OpenGL),
			specular_trans: Some(0.5),
			tile: Some(false),
			uv_offset: Some(Vec2::new(0., 0.)),
			uv_scale: Some(Vec2::new(1., 1.)),
			path: None,
		}
	}

	/// Creates a new [`StandardMaterial`] from the [`MaterialToml`]'s settings and textures.
	/// # Panics
	/// If the path field is `None` or has no parent.
	pub fn load(&self, asset_server: &AssetServer) -> StandardMaterial {
		let mut descriptor = ImageSamplerDescriptor::default();
		let mut descriptor_changed = false;
		let dir = self.dir();

		let fn_asset_path = |stem: &'static str| -> Option<PathBuf> {
			Some(
				dir?.join(stem)
					.with_extension(self.extension.as_ref().map(<String as AsRef<str>>::as_ref).unwrap_or("png")),
			)
		};

		if self.tile == Some(true) {
			descriptor.address_mode_u = ImageAddressMode::Repeat;
			descriptor.address_mode_v = ImageAddressMode::Repeat;
			descriptor.address_mode_w = ImageAddressMode::Repeat;
			descriptor_changed = true;
		}

		//now arc it!
		//this will let us safely "extend" the lifetime of the descriptor
		let descriptor_arc: Option<Arc<ImageSamplerDescriptor>> = if descriptor_changed { Some(Arc::new(descriptor)) } else { None };

		//*
		let fn_load = |stem: &'static str| {
			fn_asset_path(stem).map(|path|
			//if we have a non-default ImageSamplerDescriptor,
			//we need to do some funky stuff to safely send it without degenerating the closure into an FnOnce implementer
			if let Some(ref descriptor_ref) = descriptor_arc {
				let descriptor_send = Arc::clone(descriptor_ref);

				asset_server.load_with_settings(path, move |settings: &mut ImageLoaderSettings| {
					settings.sampler = ImageSampler::Descriptor(descriptor_send.as_ref().clone());
				})
			} else {
				asset_server.load(path)
			}
			)
		};

		//create the base material for mutating
		let mut material = StandardMaterial {
			base_color_texture: fn_load("color"),
			reflectance: self.reflectance.unwrap_or(0.5),

			..default()
		};

		//ambient occlusion
		if let Some(true) = self.ao {
			material.occlusion_texture = fn_load("ao");
		}

		//clearcoat
		if let Some(clearcoat) = self.clearcoat {
			material.clearcoat = clearcoat;

			#[cfg(feature = "pbr_multi_layer_material_textures")]
			{
				material.clearcoat_texture = fn_load("clearcoat");
			}

			#[cfg(feature = "pbr_multi_layer_material_textures")]
			match self.clearcoat_normal {
				Some(MaterialTomlClearcoatMode::BaseNormal) => material.clearcoat_normal_texture = fn_load("normal"),
				Some(MaterialTomlClearcoatMode::CustomNormal) => material.clearcoat_normal_texture = fn_load("clearcoat_normal"),
				None => {}
			}

			if let Some(clearcoat_rough) = self.clearcoat_rough {
				material.clearcoat_perceptual_roughness = clearcoat_rough;

				#[cfg(feature = "pbr_multi_layer_material_textures")]
				{
					material.clearcoat_roughness_texture = fn_load("clearcoat_rough");
				}
			}
		}

		//base color
		if let Some((red, green, blue, alpha_opt)) = self.color {
			material.base_color = Color::linear_rgba(red, green, blue, alpha_opt.unwrap_or(1.));
		}

		//depth via height map
		if let Some(depth) = self.depth {
			material.depth_map = if self.depth_hq == Some(true) {
				//considered hq because of the default sampling
				fn_load("depth")
			} else {
				//load the depth map with nearest-neighbor sampling to save fps
				fn_asset_path("depth").map(|path|
				//if we have a non-default ImageSamplerDescriptor,
				//we need to do some funky stuff to safely send it without degenerating the closure into an FnOnce implementer
				//mag_filter: ImageFilterMode::Nearest,
				//min_filter: ImageFilterMode::Nearest,
				//mipmap_filter: ImageFilterMode::Nearest,
				if let Some(ref descriptor_ref) = descriptor_arc {
					let descriptor_send = Arc::clone(descriptor_ref);

					asset_server.load_with_settings(path, move |settings: &mut ImageLoaderSettings| {
						let mut descriptor = descriptor_send.as_ref().to_owned();
						descriptor.mag_filter = ImageFilterMode::Nearest;
						descriptor.min_filter = ImageFilterMode::Nearest;
						descriptor.mipmap_filter = ImageFilterMode::Nearest;
						settings.sampler = ImageSampler::Descriptor(descriptor);
					})
				} else {
					//TODO: the else case here does not properly set the filtering modes for perf!
					//see above for proper setup!
					asset_server.load_with_settings(path, move |settings: &mut ImageLoaderSettings| {
						settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::nearest());
					})
				}
				)
			};

			material.parallax_mapping_method = match self.depth_method {
				None => ParallaxMappingMethod::Occlusion,
				Some(max_layers) => ParallaxMappingMethod::Relief { max_steps: max_layers },
			};

			material.max_parallax_layer_count = self.depth_layers.unwrap_or(16.);
			material.parallax_depth_scale = depth;
		}

		//emissive lighting
		match (self.emissive, self.emissive_color) {
			(None, None) | (Some(false), _) => {}

			(None | Some(true), color_option) => {
				if let Some((red, green, blue)) = color_option {
					material.emissive = LinearRgba::new(red, green, blue, 1.);
				} else {
					material.emissive = LinearRgba::WHITE;
				}

				material.emissive_exposure_weight = self.emissive_exposure.unwrap_or(1.0);
				material.emissive_texture = fn_load("emissive");
			}
		}

		//normals
		if let Some(normal_dir) = self.normal {
			material.flip_normal_map_y = normal_dir.should_flip();
			material.normal_map_texture = fn_load("normal");
		}

		//rough & metal
		match [self.rough, self.metal] {
			[None, None] => {}

			[rough, metal] => {
				material.metallic = metal.unwrap_or(0.);
				material.metallic_roughness_texture = fn_load("combo_0rm");
				material.perceptual_roughness = rough.unwrap_or(1.);
			}
		}

		//specular transmission
		if let Some(specular_trans) = self.specular_trans {
			material.specular_transmission = specular_trans;

			#[cfg(feature = "pbr_transmission_textures")]
			{
				material.specular_transmission_texture = fn_load("specular_trans");
			}
		}

		if let Some(uv_offset) = self.uv_scale {
			material.uv_transform.translation = uv_offset;
		}

		if let Some(uv_scale) = self.uv_scale {
			material.uv_transform.matrix2 = Mat2::from_cols(Vec2::X * uv_scale.x, Vec2::Y * uv_scale.y);
		}

		material
	}

	/// If pointed to a directory, tries to load the `material.toml` file in that directory.
	pub fn new(path: impl Into<PathBuf>) -> Result<Self, MaterialTomlError> {
		let mut path = path.into();

		if path.extension().is_none() {
			path = path.join("material.toml");
		}

		let mut mat_toml = toml::from_str::<MaterialToml>(&fs::read_to_string(if path.is_relative() {
			PathBuf::from("assets").join(&path)
		} else {
			path.clone()
		})?)?;

		//if relative, should not have assets prefixed
		mat_toml.path = Some(path);

		Ok(mat_toml)
	}

	/// Writes a serialized material toml using its path field.
	pub fn save(&self) -> Result<(), MaterialTomlError> {
		fs::write(self.toml_path().ok_or(MaterialTomlError::MissingPath)?, toml::to_string(&self)?)?;

		Ok(())
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MaterialTomlClearcoatMode {
	/// Use a seperate normal map for clearcoat.
	#[default]
	CustomNormal,

	/// Use the base texture's normal map.
	BaseNormal,
}

#[derive(Debug, thiserror::Error)]
pub enum MaterialTomlError {
	#[error("missing path and/or dir fields")]
	MissingPath,

	#[error("StdIo error")]
	StdIo(#[from] std::io::Error),

	#[error("toml serde(de) error")]
	TomlDeserialization(#[from] toml::de::Error),

	#[error("toml serde(ser) error")]
	TomlSerialization(#[from] toml::ser::Error),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MaterialTomlNormalsYDir {
	/// Good to go.
	#[default]
	OpenGL,

	/// Needs flipping.
	DirectX,
}

impl MaterialTomlNormalsYDir {
	pub fn should_flip(self) -> bool {
		match self {
			MaterialTomlNormalsYDir::OpenGL => false,
			MaterialTomlNormalsYDir::DirectX => true,
		}
	}
}

// #[test]
// fn generate_sample_material_toml() {
// 	fs::write(
// 		std::env::current_dir().unwrap().join("sample_material.toml"),
// 		toml::to_string(&MaterialToml::example()).unwrap(),
// 	)
// 	.unwrap();
// }
