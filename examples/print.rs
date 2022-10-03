fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let bytes = std::fs::read(&filename).unwrap();
    let (gltf, b): (
        goth_gltf::Gltf<goth_gltf::default_extensions::Extensions>,
        _,
    ) = goth_gltf::Gltf::from_bytes(&bytes).unwrap();
    println!("{:#?}", gltf);
}
