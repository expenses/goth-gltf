fn main() {
    let filename = std::env::args().nth(1).unwrap();

    let json_filename = std::env::args().nth(2).unwrap();
    let buffer_filename = std::env::args().nth(3).unwrap();

    let bytes = std::fs::read(&filename).unwrap();

    // Check for the 4-byte magic.
    if !bytes.starts_with(b"glTF") {
        panic!();
    }

    // There's always a json chunk at the start:
    // https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#structured-json-content

    let json_chunk_length = u32::from_le_bytes(bytes[12..16].try_into().unwrap());

    let json_chunk_end = 20 + json_chunk_length as usize;

    let json_chunk_bytes = &bytes[20..20 + json_chunk_length as usize];

    let binary_buffer = &bytes[json_chunk_end + 8..];

    std::fs::write(&json_filename, json_chunk_bytes).unwrap();
    std::fs::write(&buffer_filename, binary_buffer).unwrap();
}
