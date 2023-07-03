use crate::*;
use std::borrow::Cow;
use std::collections::HashMap;
use thiserror::Error;

pub trait MeshOptCompressionExtension {
    fn ext_meshopt_compression(&self) -> Option<extensions::ExtMeshoptCompression>;
}

impl MeshOptCompressionExtension for crate::default_extensions::BufferViewExtensions {
    fn ext_meshopt_compression(&self) -> Option<extensions::ExtMeshoptCompression> {
        self.ext_meshopt_compression
    }
}

impl MeshOptCompressionExtension for () {
    fn ext_meshopt_compression(&self) -> Option<extensions::ExtMeshoptCompression> {
        None
    }
}

fn unsigned_short_to_float(short: u16) -> f32 {
    short as f32 / 65535.0
}

fn unsigned_byte_to_float(byte: u8) -> f32 {
    byte as f32 / 255.0
}

fn signed_byte_to_float(byte: i8) -> f32 {
    (byte as f32 / 127.0).max(-1.0)
}

fn signed_short_to_float(short: i16) -> f32 {
    (short as f32 / 32767.0).max(-1.0)
}

fn byte_stride<E: Extensions>(
    accessor: &crate::Accessor,
    buffer_view: &crate::BufferView<E>,
) -> usize
where
    E::BufferViewExtensions: MeshOptCompressionExtension,
{
    buffer_view
        .extensions
        .ext_meshopt_compression()
        .map(|ext| ext.byte_stride)
        .or(buffer_view.byte_stride)
        .unwrap_or_else(|| {
            accessor.component_type.byte_size() * accessor.accessor_type.num_components()
        })
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Accessor is missing buffer view")]
    AccessorMissingBufferView,
    #[error("Buffer view index {0} out of bounds")]
    BufferViewIndexOutOfBounds(usize),
    #[error("Accessor index {0} out of bounds")]
    AccessorIndexOutOfBounds(usize),
    #[error("{0}: Unsupported combination of component type, normalized and byte stride: {1:?}")]
    UnsupportedCombination(u32, (ComponentType, bool, Option<usize>)),
}

pub fn read_buffer_with_accessor<'a, E: Extensions>(
    buffer_view_map: &'a HashMap<usize, Vec<u8>>,
    gltf: &'a crate::Gltf<E>,
    accessor: &crate::Accessor,
) -> Result<(&'a [u8], Option<usize>), Error>
where
    E::BufferViewExtensions: MeshOptCompressionExtension,
{
    let buffer_view_index = accessor
        .buffer_view
        .ok_or(Error::AccessorMissingBufferView)?;
    let buffer_view = gltf
        .buffer_views
        .get(buffer_view_index)
        .ok_or(Error::BufferViewIndexOutOfBounds(buffer_view_index))?;

    let start = accessor.byte_offset;
    let end = start + accessor.count * byte_stride(accessor, buffer_view);

    let buffer_view_bytes = buffer_view_map
        .get(&buffer_view_index)
        .ok_or(Error::BufferViewIndexOutOfBounds(buffer_view_index))?;

    // Force the end of the slice to be in-bounds as either the maths for calculating
    // `end` is wrong or some files are a little odd.
    let end = end.min(buffer_view_bytes.len());

    let slice = &buffer_view_bytes[start..end];

    Ok((slice, buffer_view.byte_stride))
}

pub fn read_f32<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &crate::Accessor,
) -> Result<Cow<'a, [f32]>, Error> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None) => Cow::Borrowed(bytemuck::cast_slice(slice)),
            other => return Err(Error::UnsupportedCombination(std::line!(), other)),
        },
    )
}

pub fn read_f32x3<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &crate::Accessor,
) -> Result<Cow<'a, [[f32; 3]]>, Error> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None | Some(12)) => {
                let slice: &[f32] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(3)
                        .map(|slice| <[f32; 3]>::try_from(slice).unwrap())
                        .collect(),
                )
            }
            (ComponentType::Short, true, Some(stride)) => {
                let slice: &[i16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(stride / 2)
                        .map(|slice| std::array::from_fn(|i| signed_short_to_float(slice[i])))
                        .collect(),
                )
            }
            (ComponentType::UnsignedShort, false, Some(8)) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(4)
                        .map(move |slice| std::array::from_fn(|i| slice[i] as f32))
                        .collect(),
                )
            }
            (ComponentType::UnsignedShort, true, Some(8)) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(4)
                        .map(|slice| std::array::from_fn(|i| unsigned_short_to_float(slice[i])))
                        .collect(),
                )
            }
            (ComponentType::Byte, true, Some(stride)) => Cow::Owned(
                slice
                    .chunks(stride)
                    .map(move |slice| std::array::from_fn(|i| signed_byte_to_float(slice[i] as i8)))
                    .collect(),
            ),
            other => return Err(Error::UnsupportedCombination(std::line!(), other)),
        },
    )
}

fn read_f32x2<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &crate::Accessor,
) -> Result<Cow<'a, [[f32; 2]]>, Error> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None | Some(8)) => {
                Cow::Borrowed(bytemuck::cast_slice(slice))
            }
            (ComponentType::Float, false, Some(stride)) => {
                let slice: &[f32] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(stride / 4)
                        .map(move |slice| std::array::from_fn(|i| slice[i]))
                        .collect(),
                )
            }
            (ComponentType::UnsignedShort, true, Some(stride)) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .chunks(stride / 2)
                        .map(move |slice| {
                            std::array::from_fn(|i| unsigned_short_to_float(slice[i]))
                        })
                        .collect(),
                )
            }
            other => return Err(Error::UnsupportedCombination(std::line!(), other)),
        },
    )
}

unsafe fn cast_slice<T>(bytes: &[u8]) -> &[T] {
    std::slice::from_raw_parts(
        bytes.as_ptr() as *const T,
        bytes.len() / std::mem::size_of::<T>(),
    )
}

pub fn read_f32x4<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &crate::Accessor,
) -> Result<Cow<'a, [[f32; 4]]>, Error> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::Float, false, None) => {
                // bytemuck::cast_slice panics with an alignment issue on wasm so we just use unsafe for this.
                // todo: might be wrong.
                Cow::Borrowed(unsafe { cast_slice(slice) })
            }
            (ComponentType::UnsignedByte, true, Some(4)) => Cow::Owned(
                slice
                    .chunks(4)
                    .map(move |slice| std::array::from_fn(|i| unsigned_byte_to_float(slice[i])))
                    .collect(),
            ),
            (ComponentType::Short, true, None) => {
                let slice: &[[i16; 4]] = bytemuck::cast_slice(slice);
                Cow::Owned(
                    slice
                        .iter()
                        .map(|slice| std::array::from_fn(|i| signed_short_to_float(slice[i])))
                        .collect(),
                )
            }
            other => return Err(Error::UnsupportedCombination(std::line!(), other)),
        },
    )
}

fn read_u32<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &crate::Accessor,
) -> Result<Cow<'a, [u32]>, Error> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::UnsignedShort, false, None) => {
                let slice: &[u16] = bytemuck::cast_slice(slice);
                Cow::Owned(slice.iter().map(|&i| i as u32).collect())
            }
            (ComponentType::UnsignedInt, false, None) => Cow::Borrowed(bytemuck::cast_slice(slice)),
            other => return Err(Error::UnsupportedCombination(std::line!(), other)),
        },
    )
}

fn read_u32x4<'a>(
    slice: &'a [u8],
    byte_stride: Option<usize>,
    accessor: &crate::Accessor,
) -> Result<Cow<'a, [[u32; 4]]>, Error> {
    Ok(
        match (accessor.component_type, accessor.normalized, byte_stride) {
            (ComponentType::UnsignedByte, false, Some(4) | None) => Cow::Owned(
                slice
                    .chunks(4)
                    .map(|slice| std::array::from_fn(|i| slice[i] as u32))
                    .collect(),
            ),
            other => return Err(Error::UnsupportedCombination(std::line!(), other)),
        },
    )
}

pub struct PrimitiveReader<'a, E: Extensions> {
    gltf: &'a crate::Gltf<E>,
    pub primitive: &'a crate::Primitive,
    buffer_view_map: &'a HashMap<usize, Vec<u8>>,
}

impl<'a, E: Extensions> PrimitiveReader<'a, E>
where
    E::BufferViewExtensions: MeshOptCompressionExtension,
{
    pub fn new(
        gltf: &'a crate::Gltf<E>,
        primitive: &'a crate::Primitive,
        buffer_view_map: &'a HashMap<usize, Vec<u8>>,
    ) -> Self {
        Self {
            gltf,
            primitive,
            buffer_view_map,
        }
    }

    pub fn read_indices(&self) -> Result<Option<Cow<'a, [u32]>>, Error> {
        let accessor_index = match self.primitive.indices {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_u32(slice, byte_stride, accessor)?))
    }

    pub fn read_positions(&self) -> Result<Option<Cow<'a, [[f32; 3]]>>, Error> {
        let accessor_index = match self.primitive.attributes.position {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x3(slice, byte_stride, accessor)?))
    }

    pub fn read_normals(&self) -> Result<Option<Cow<'a, [[f32; 3]]>>, Error> {
        let accessor_index = match self.primitive.attributes.normal {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x3(slice, byte_stride, accessor)?))
    }

    pub fn read_uvs(&self) -> Result<Option<Cow<'a, [[f32; 2]]>>, Error> {
        let accessor_index = match self.primitive.attributes.texcoord_0 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x2(slice, byte_stride, accessor)?))
    }

    pub fn read_second_uvs(&self) -> Result<Option<Cow<'a, [[f32; 2]]>>, Error> {
        let accessor_index = match self.primitive.attributes.texcoord_1 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x2(slice, byte_stride, accessor)?))
    }

    pub fn read_joints(&self) -> Result<Option<Cow<'a, [[u32; 4]]>>, Error> {
        let accessor_index = match self.primitive.attributes.joints_0 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;

        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_u32x4(slice, byte_stride, accessor)?))
    }

    pub fn read_weights(&self) -> Result<Option<Cow<'a, [[f32; 4]]>>, Error> {
        let accessor_index = match self.primitive.attributes.weights_0 {
            Some(index) => index,
            None => return Ok(None),
        };

        let accessor = self
            .gltf
            .accessors
            .get(accessor_index)
            .ok_or(Error::AccessorIndexOutOfBounds(accessor_index))?;
        let (slice, byte_stride) =
            read_buffer_with_accessor(self.buffer_view_map, self.gltf, accessor)?;

        Ok(Some(read_f32x4(slice, byte_stride, accessor)?))
    }
}
