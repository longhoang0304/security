pub fn xor_bytes(data: &mut Vec<u8>, key: Vec<u8>) {
    let key_size = key.len();

    for (idx, byte) in data.iter_mut().enumerate() {
        *byte ^= key[idx % key_size]
    }
}
