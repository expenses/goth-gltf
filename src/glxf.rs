use nanoserde::DeJson;

#[derive(Debug, DeJson)]
pub struct Glxf<E: crate::Extensions> {
    #[nserde(default)]
    pub assets: Vec<Asset>,
    #[nserde(default)]
    pub nodes: Vec<crate::Node<E>>,
    #[nserde(default)]
    pub cameras: Vec<crate::Camera>,
    #[nserde(default)]
    pub extensions: E::RootExtensions,
    #[nserde(default)]
    pub scenes: Vec<crate::Scene>,
    #[nserde(default)]
    pub scene: usize,
}

#[derive(Debug, DeJson)]
pub struct Asset {
    pub uri: String,
    pub scene: Option<String>,
    pub nodes: Option<Vec<String>>,
    #[nserde(default)]
    pub transform: AssetTransform,
    #[cfg(feature = "names")]
    pub name: Option<String>,
}

#[derive(Debug, DeJson)]
pub enum AssetTransform {
    #[nserde(rename = "none")]
    None,
    #[nserde(rename = "local")]
    Local,
    #[nserde(rename = "global")]
    Global,
}

impl Default for AssetTransform {
    fn default() -> Self {
        Self::Global
    }
}
