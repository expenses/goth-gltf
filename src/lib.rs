use nanoserde::DeJson;

#[derive(Debug, DeJson)]
pub struct Gltf {
    #[nserde(default)]
    pub images: Vec<Image>,
    #[nserde(default)]
    pub textures: Vec<Texture>,
    #[nserde(default)]
    pub materials: Vec<Material>,
    #[nserde(default)]
    pub buffers: Vec<Buffer>,
    #[nserde(rename = "bufferViews")]
    #[nserde(default)]
    pub buffer_views: Vec<BufferView>,
    #[nserde(default)]
    pub accessors: Vec<Accessor>,
    #[nserde(default)]
    pub meshes: Vec<Mesh>,
    #[nserde(default)]
    pub animations: Vec<Animation>,
    #[nserde(default)]
    pub nodes: Vec<Node>,
    #[nserde(default)]
    pub skins: Vec<Skin>,
}

impl Gltf {
    pub fn from_str(string: &str) -> Result<Self, nanoserde::DeJsonErr> {
        Self::deserialize_json(string)
    }
}

#[derive(Debug, DeJson)]
pub struct Skin {
    #[nserde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<usize>,
    pub skeleton: Option<usize>,
    pub joints: Vec<usize>,
}

#[derive(Debug, DeJson)]
pub struct Animation {
    pub channels: Vec<Channel>,
    pub samplers: Vec<AnimationSampler>,
}

#[derive(Debug, DeJson)]
pub struct Channel {
    pub sampler: usize,
    pub target: Target,
}

#[derive(Debug, DeJson)]
pub struct Target {
    pub node: Option<usize>,
    pub path: TargetPath,
}

#[derive(Debug, DeJson)]
pub struct AnimationSampler {
    pub input: usize,
    #[nserde(default)]
    pub interpolation: Interpolation,
    pub output: usize,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub enum Interpolation {
    #[nserde(rename = "LINEAR")]
    Linear,
    #[nserde(rename = "STEP")]
    Step,
    #[nserde(rename = "CUBICSPLINE")]
    CubicSpline,
}

impl Default for Interpolation {
    fn default() -> Self {
        Self::Linear
    }
}

#[derive(Debug, DeJson)]
pub enum TargetPath {
    #[nserde(rename = "translation")]
    Translation,
    #[nserde(rename = "rotation")]
    Rotation,
    #[nserde(rename = "scale")]
    Scale,
    #[nserde(rename = "weights")]
    Weights,
}

#[derive(Debug, DeJson)]
pub struct Buffer {
    pub uri: Option<String>,
    #[nserde(rename = "byteLength")]
    pub byte_length: usize,
}

#[derive(Debug, DeJson)]
pub struct Node {
    pub camera: Option<usize>,
    #[nserde(default)]
    pub children: Vec<usize>,
    pub skin: Option<usize>,
    pub matrix: Option<[f32; 16]>,
    pub mesh: Option<usize>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Option<[f32; 3]>,
    pub translation: Option<[f32; 3]>,
    // missing: weights
}

impl Node {
    pub fn transform(&self) -> NodeTransform {
        match self.matrix {
            Some(matrix) => match (self.translation, self.rotation, self.scale) {
                // If both a matrix and a full transform set is specified, then just use the transform.
                (Some(translation), Some(rotation), Some(scale)) => NodeTransform::Set {
                    translation,
                    rotation,
                    scale,
                },
                _ => NodeTransform::Matrix(matrix),
            },
            None => {
                let translation = self.translation.unwrap_or([0.0; 3]);
                let rotation = self.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                let scale = self.scale.unwrap_or([1.0; 3]);
                NodeTransform::Set {
                    translation,
                    rotation,
                    scale,
                }
            }
        }
    }
}

pub enum NodeTransform {
    Matrix([f32; 16]),
    Set {
        translation: [f32; 3],
        rotation: [f32; 4],
        scale: [f32; 3],
    },
}

#[derive(Debug, DeJson)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
    // missing: weights
}

#[derive(Debug, DeJson)]
pub struct Primitive {
    pub attributes: Attributes,
    pub indices: Option<usize>,
    pub material: Option<usize>,
    // missing: mode, targets
}

#[derive(Debug, DeJson)]
pub struct Attributes {
    #[nserde(rename = "POSITION")]
    pub position: Option<usize>,
    #[nserde(rename = "TANGENT")]
    pub tangent: Option<usize>,
    #[nserde(rename = "NORMAL")]
    pub normal: Option<usize>,
    #[nserde(rename = "TEXCOORD_0")]
    pub texcoord_0: Option<usize>,
    #[nserde(rename = "JOINTS_0")]
    pub joints_0: Option<usize>,
    #[nserde(rename = "WEIGHTS_0")]
    pub weights_0: Option<usize>,
}

#[derive(Debug, DeJson)]
pub struct Image {
    pub uri: Option<String>,
    #[nserde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[nserde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
}

#[derive(Debug, DeJson)]
pub struct Texture {
    pub sampler: Option<usize>,
    pub source: Option<usize>,
    #[nserde(default)]
    pub extensions: TextureExtensions,
}

#[derive(Debug, Default, DeJson)]
pub struct TextureExtensions {
    #[nserde(rename = "KHR_texture_basisu")]
    pub khr_texture_basisu: Option<KhrTextureBasisu>,
}

#[derive(Debug, DeJson)]
pub struct KhrTextureBasisu {
    pub source: usize,
}

#[derive(Debug, DeJson)]
pub struct BufferView {
    pub buffer: usize,
    #[nserde(rename = "byteOffset")]
    #[nserde(default)]
    pub byte_offset: usize,
    #[nserde(rename = "byteLength")]
    pub byte_length: usize,
    #[nserde(rename = "byteStride")]
    pub byte_stride: Option<usize>,
    #[nserde(default)]
    pub extensions: BufferViewExtensions,
}

#[derive(Debug, DeJson, Default)]
pub struct BufferViewExtensions {
    #[nserde(rename = "EXT_meshopt_compression")]
    ext_meshopt_compression: Option<ExtMeshoptCompression>,
}

#[derive(Debug, DeJson)]
pub struct ExtMeshoptCompression {
    pub buffer: usize,
    #[nserde(rename = "byteOffset")]
    #[nserde(default)]
    pub byte_offset: usize,
    #[nserde(rename = "byteLength")]
    pub byte_length: usize,
    #[nserde(rename = "byteStride")]
    pub byte_stride: usize,
    pub mode: ExtMeshoptCompressionMode,
    pub count: usize,
    #[nserde(default)]
    pub filter: ExtMeshoptCompressionFilter,
}

#[derive(Debug, DeJson)]
pub enum ExtMeshoptCompressionMode {
    #[nserde(rename = "ATTRIBUTES")]
    Attributes,
    #[nserde(rename = "TRIANGLES")]
    Triangles,
    #[nserde(rename = "INDICES")]
    Indices,
}

#[derive(Debug, DeJson)]
pub enum ExtMeshoptCompressionFilter {
    #[nserde(rename = "NONE")]
    None,
    #[nserde(rename = "OCTAHEDRAL")]
    Octahedral,
    #[nserde(rename = "QUATERNION")]
    Quaternion,
    #[nserde(rename = "EXPONENTIAL")]
    Exponential,
}

impl Default for ExtMeshoptCompressionFilter {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, DeJson)]
pub struct Accessor {
    #[nserde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    #[nserde(rename = "byteOffset")]
    #[nserde(default)]
    pub byte_offset: usize,
    #[nserde(rename = "componentType")]
    pub component_type: ComponentType,
    #[nserde(default)]
    pub normalized: bool,
    pub count: usize,
    #[nserde(rename = "type")]
    pub accessor_type: AccessorType,
    // missing: sparse, min, max
    // min and max are [f32; T] where T is the dimension of the accessor.
}

impl Accessor {
    pub fn byte_length(&self, buffer_view: &BufferView) -> usize {
        self.count
            * buffer_view.byte_stride.unwrap_or_else(|| {
                self.component_type.byte_size() * self.accessor_type.num_components()
            })
    }
}

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    UnsignedByte,
    Byte,
    UnsignedShort,
    Short,
    UnsignedInt,
    Float,
}

impl ComponentType {
    pub fn byte_size(&self) -> usize {
        match self {
            Self::UnsignedByte | Self::Byte => 1,
            Self::UnsignedShort | Self::Short => 2,
            Self::UnsignedInt | Self::Float => 4,
        }
    }
}

impl DeJson for ComponentType {
    fn de_json(
        state: &mut nanoserde::DeJsonState,
        input: &mut core::str::Chars,
    ) -> Result<Self, nanoserde::DeJsonErr> {
        let ty = match &state.tok {
            nanoserde::DeJsonTok::U64(ty) => match ty {
                5120 => Self::Byte,
                5121 => Self::UnsignedByte,
                5122 => Self::Short,
                5123 => Self::UnsignedShort,
                5125 => Self::UnsignedInt,
                5126 => Self::Float,
                other => {
                    return Err(nanoserde::DeJsonErr {
                        msg: format!("Invalid component type '{}'", other),
                        line: state.line,
                        col: state.col,
                    })
                }
            },
            other => {
                return Err(nanoserde::DeJsonErr {
                    msg: format!("Unexpected token {:?}, expected U64", other),
                    line: state.line,
                    col: state.col,
                })
            }
        };

        state.next_tok(input)?;

        Ok(ty)
    }
}

#[derive(Debug, DeJson)]
pub enum AccessorType {
    #[nserde(rename = "SCALAR")]
    Scalar,
    #[nserde(rename = "VEC2")]
    Vec2,
    #[nserde(rename = "VEC3")]
    Vec3,
    #[nserde(rename = "VEC4")]
    Vec4,
    #[nserde(rename = "MAT2")]
    Mat2,
    #[nserde(rename = "MAT3")]
    Mat3,
    #[nserde(rename = "MAT4")]
    Mat4,
}

impl AccessorType {
    pub fn num_components(&self) -> usize {
        match self {
            Self::Scalar => 1,
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 | Self::Mat2 => 4,
            Self::Mat3 => 9,
            Self::Mat4 => 16,
        }
    }
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct Material {
    #[nserde(rename = "pbrMetallicRoughness")]
    #[nserde(default)]
    pub pbr_metallic_roughness: PbrMetallicRoughness,
    #[nserde(rename = "normalTexture")]
    pub normal_texture: Option<TextureInfo>,
    #[nserde(rename = "occlusionTexture")]
    pub occlusion_texture: Option<TextureInfo>,
    #[nserde(rename = "emissiveTexture")]
    pub emissive_texture: Option<TextureInfo>,
    #[nserde(rename = "emissiveFactor")]
    #[nserde(default)]
    pub emissive_factor: [f32; 3],
    #[nserde(rename = "alphaMode")]
    #[nserde(default)]
    pub alpha_mode: AlphaMode,
    #[nserde(rename = "alphaCutoff")]
    #[nserde(default = "0.5")]
    pub alpha_cutoff: f32,
    #[nserde(rename = "doubleSided")]
    #[nserde(default)]
    pub double_sided: bool,
    #[nserde(default)]
    pub extensions: MaterialExtensions,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub enum AlphaMode {
    #[nserde(rename = "OPAQUE")]
    Opaque,
    #[nserde(rename = "MASK")]
    Mask,
    #[nserde(rename = "BLEND")]
    Blend,
}

impl Default for AlphaMode {
    fn default() -> Self {
        Self::Opaque
    }
}

#[derive(Debug, DeJson, Default, Clone, Copy)]
pub struct MaterialExtensions {
    #[nserde(rename = "KHR_materials_sheen")]
    pub khr_materials_sheen: Option<KhrMaterialsSheen>,
    #[nserde(rename = "KHR_materials_emissive_strength")]
    pub khr_materials_emissive_strength: Option<KhrMaterialsEmissiveStrength>,
    #[nserde(rename = "KHR_materials_unlit")]
    pub khr_materials_unlit: Option<KhrMaterialsUnlit>,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrMaterialsSheen {
    #[nserde(rename = "sheenColorFactor")]
    #[nserde(default)]
    pub sheen_color_factor: [f32; 3],
    #[nserde(rename = "sheenColorTexture")]
    pub sheen_color_texture: Option<TextureInfo>,
    #[nserde(rename = "sheenRoughnessFactor")]
    #[nserde(default)]
    pub sheen_roughness_factor: f32,
    #[nserde(rename = "sheenRoughnessTexture")]
    pub sheen_roughness_texture: Option<TextureInfo>,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrMaterialsEmissiveStrength {
    #[nserde(rename = "emissiveStrength")]
    #[nserde(default = "1.0")]
    pub emissive_strength: f32,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct KhrMaterialsUnlit {}

#[derive(Debug, DeJson, Default, Clone, Copy)]
pub struct PbrMetallicRoughness {
    #[nserde(rename = "baseColorFactor")]
    #[nserde(default = "[1.0, 1.0, 1.0, 1.0]")]
    pub base_color_factor: [f32; 4],
    #[nserde(rename = "baseColorTexture")]
    pub base_color_texture: Option<TextureInfo>,
    #[nserde(rename = "metallicFactor")]
    #[nserde(default = "1.0")]
    pub metallic_factor: f32,
    #[nserde(rename = "roughnessFactor")]
    #[nserde(default = "1.0")]
    pub roughness_factor: f32,
    #[nserde(rename = "metallicRoughnessTexture")]
    pub metallic_roughness_texture: Option<TextureInfo>,
}

#[derive(Debug, DeJson, Clone, Copy)]
pub struct TextureInfo {
    pub index: usize,
    #[nserde(rename = "texCoord")]
    #[nserde(default)]
    pub tex_coord: usize,
    #[nserde(default)]
    pub extensions: TextureInfoExtensions,
}

#[derive(Debug, DeJson, Default, Clone, Copy)]
pub struct TextureInfoExtensions {
    #[nserde(rename = "KHR_texture_transform")]
    pub khr_texture_transform: Option<KhrTextureTransform>,
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
