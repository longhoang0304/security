pub const IMAGE_NUMBER_OF_DIRECTORY_ENTRIES: usize = 16;

pub struct DataDirectory {
    virtual_address: u32,
    size: u32,
}
