use std::{fs};
use std::error::Error;
use std::io::{Read};
use crate::pe::headers::{DOSHeader, Parsable};

pub fn read_and_parse(file_name: &str) -> Result<DOSHeader, Box<dyn Error>> {
    let mut pe_file = fs::File::open(file_name)?;
    let mut data = Vec::new();

    pe_file.by_ref().take(49 * 2).read_to_end(&mut data)?;

    let dos_header = DOSHeader::from_bytes(&data).unwrap();

    Ok(dos_header)
}