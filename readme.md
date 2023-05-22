Goth-gltf aims to be a low-level, unopinionated reader for gltf files.

## In comparison with [gltf-rs], it:

- Represents the gltf JSON structure transparently
- Uses nanoserde instead of serde
- Supports a wider range of extensions
- Has no code specific for loading images or reading attributes out of buffers

## Extensions Implemented

- `KHR_lights_punctual`
- `KHR_materials_emissive_strength`
- `KHR_materials_ior`
- `KHR_materials_sheen`
- `KHR_materials_unlit`
- `KHR_texture_basisu`
- `KHR_texture_transform`
- `EXT_mesh_gpu_instancing`
- `EXT_meshopt_compression`
- `MSFT_lod`
- `MSFT_screencoverage`

[gltf-rs]: https://github.com/gltf-rs/gltf
