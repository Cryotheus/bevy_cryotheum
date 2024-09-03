# Cryotheum's Bevy Suite
A collection of plugins and utilities for games I make on the Bevy game engine.

# Work in Progress
This is most definitely not production-ready.

# Manifest Features
|               Feature               | Default | Dependencies | Notes                                                                                                                          |
|:-----------------------------------:|:-------:|--------------|--------------------------------------------------------------------------------------------------------------------------------|
|             `arrayvec`              |    ✅    | `arrayvec`   | For disabling the plugins module.                                                                                              |
|             `smallvec`              |    ✅    | `smallvec`   | Adds a plugin for steamworks integration.                                                                                      |
|               `serde`               |         |              | Enables the `serde` feature on dependencies, and enables the `material_toml` module.                                           |
|          `dynamic_linking`          |         |              | Enables bevy's `dynamic_linking` feature                                                                                       |
| `pbr_multi_layer_material_textures` |         |              | Enables bevy's `pbr_multi_layer_material_textures` feature and allows `MaterialToml` to load clearcoat textures.               |
|     `pbr_transmission_textures`     |         |              | Enables bevy's `pbr_transmission_textures` feature and allows `MaterialToml` to load specular transmission textures. textures. |

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bevy_cryotheum` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
