fn main() {
    let path = std::env::args().nth(1).unwrap();
    let path = std::path::Path::new(&path);
    let bytes = std::fs::read(&path).unwrap();
    let (gltf, _): (
        goth_gltf::Gltf<goth_gltf::default_extensions::Extensions>,
        _,
    ) = goth_gltf::Gltf::from_bytes(&bytes).unwrap();

    assert_eq!(gltf.buffers.len(), 1);
    let buffer = std::fs::read(path.with_file_name(gltf.buffers[0].uri.as_ref().unwrap())).unwrap();

    let get_buffer_slice = |accessor: &goth_gltf::Accessor| {
        let bv = &gltf.buffer_views[accessor.buffer_view.unwrap()];
        assert_eq!(bv.byte_stride, None);
        &buffer[bv.byte_offset..bv.byte_offset + bv.byte_length][accessor.byte_offset..]
    };

    let mesh = &gltf.meshes[0];
    assert_eq!(mesh.primitives.len(), 1);
    let primitive = &mesh.primitives[0];
    let positions = &gltf.accessors[primitive.attributes.position.unwrap()];
    let expected = (
        goth_gltf::ComponentType::Float,
        goth_gltf::AccessorType::Vec3,
        false,
    );
    assert_eq!(
        (
            positions.component_type,
            positions.accessor_type,
            positions.normalized
        ),
        expected
    );
    let positions_slice = get_buffer_slice(positions);
    let normals = &gltf.accessors[primitive.attributes.normal.unwrap()];
    assert_eq!(
        (
            normals.component_type,
            normals.accessor_type,
            normals.normalized
        ),
        expected
    );
    let normals_slice = get_buffer_slice(normals);

    let indices = &gltf.accessors[primitive.indices.unwrap()];
    let indices_slice = get_buffer_slice(indices);
    dbg!(indices);

    use std::io::Write;
    let mut output = std::io::BufWriter::new(std::fs::File::create("output.bin").unwrap());

    let mut write_val = |val: u32| {
        output.write_all(&val.to_le_bytes()).unwrap();
    };

    write_val(indices.count as _);
    write_val(positions.count as _);
    output.write_all(indices_slice);
    output.write_all(positions_slice);
    output.write_all(normals_slice);
}
