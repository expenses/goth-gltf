use crate::{Extensions, TextureInfo};
use nanoserde::DeJson;

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrTextureBasisu {
    pub source: usize,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrTextureTransform {
    #[nserde(default)]
    pub offset: [f32; 2],
    #[nserde(default)]
    pub rotation: f32,
    #[nserde(default = "[1.0, 1.0]")]
    pub scale: [f32; 2],
    #[nserde(rename = "texCoord")]
    #[nserde(default)]
    pub tex_coord: usize,
}

#[derive(Debug, DeJson, Clone)]
pub struct KhrMaterialsSheen<E: Extensions> {
    #[nserde(rename = "sheenColorFactor")]
    #[nserde(default)]
    pub sheen_color_factor: [f32; 3],
    #[nserde(rename = "sheenColorTexture")]
    pub sheen_color_texture: Option<TextureInfo<E>>,
    #[nserde(rename = "sheenRoughnessFactor")]
    #[nserde(default)]
    pub sheen_roughness_factor: f32,
    #[nserde(rename = "sheenRoughnessTexture")]
    pub sheen_roughness_texture: Option<TextureInfo<E>>,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrMaterialsEmissiveStrength {
    #[nserde(rename = "emissiveStrength")]
    #[nserde(default = "1.0")]
    pub emissive_strength: f32,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrMaterialsUnlit {}

#[derive(Debug, DeJson, Clone)]
pub struct KhrLightsPunctual {
    #[nserde(default)]
    pub lights: Vec<Light>,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct Light {
    #[nserde(default = "[1.0, 1.0, 1.0]")]
    pub color: [f32; 3],
    #[nserde(default = "1.0")]
    pub intensity: f32,
    #[nserde(rename = "type")]
    pub ty: LightType,
    pub spot: Option<LightSpot>,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub enum LightType {
    #[nserde(rename = "point")]
    Point,
    #[nserde(rename = "directional")]
    Directional,
    #[nserde(rename = "spot")]
    Spot,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct LightSpot {
    #[nserde(rename = "innerConeAngle")]
    #[nserde(default)]
    pub inner_cone_angle: f32,
    #[nserde(rename = "outerConeAngle")]
    #[nserde(default = "std::f32::consts::FRAC_PI_4")]
    pub outer_cone_angle: f32,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrMaterialsIor {
    #[nserde(default = "1.5")]
    pub ior: f32,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct ExtMeshoptCompression {
    pub buffer: usize,
    #[nserde(rename = "byteOffset")]
    #[nserde(default)]
    pub byte_offset: usize,
    #[nserde(rename = "byteLength")]
    pub byte_length: usize,
    #[nserde(rename = "byteStride")]
    pub byte_stride: usize,
    pub mode: CompressionMode,
    pub count: usize,
    #[nserde(default)]
    pub filter: CompressionFilter,
}

#[derive(Debug, DeJson, PartialEq, Eq, Clone, Copy)]
pub enum CompressionMode {
    #[nserde(rename = "ATTRIBUTES")]
    Attributes,
    #[nserde(rename = "TRIANGLES")]
    Triangles,
    #[nserde(rename = "INDICES")]
    Indices,
}

#[derive(Debug, DeJson, PartialEq, Eq, Clone, Copy)]
pub enum CompressionFilter {
    #[nserde(rename = "NONE")]
    None,
    #[nserde(rename = "OCTAHEDRAL")]
    Octahedral,
    #[nserde(rename = "QUATERNION")]
    Quaternion,
    #[nserde(rename = "EXPONENTIAL")]
    Exponential,
}

impl Default for CompressionFilter {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct ExtMeshoptCompressionBuffer {
    #[nserde(default)]
    pub fallback: bool,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct ExtMeshGpuInstancing {
    pub attributes: ExtMeshGpuInstancingAttributes,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct ExtMeshGpuInstancingAttributes {
    #[nserde(rename = "ROTATION")]
    pub rotation: usize,
    #[nserde(rename = "SCALE")]
    pub scale: usize,
    #[nserde(rename = "TRANSLATION")]
    pub translation: usize,
}
