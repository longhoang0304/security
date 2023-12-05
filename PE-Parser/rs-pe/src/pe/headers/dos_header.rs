use std::fmt::{Display, Formatter};
use crate::pe::headers::parsable::{Parsable, ParseError};
use crate::helpers::{take_bytes_start, take_words_array_start};

macro_rules! dos_header_format {() => (r#"DOS-Header{{
    e_magic: {:X}
    e_cblp: {:X}
    e_cp: {:X}
    e_crlc: {:X}
    e_cparhdr: {:X}
    e_minalloc: {:X}
    e_maxalloc: {:X}
    e_ss: {:X}
    e_sp: {:X}
    e_csum: {:X}
    e_ip: {:X}
    e_cs: {:X}
    e_lfarlc: {:X}
    e_ovno: {:X}
    e_res: {:?}
    e_oemid: {:X}
    e_oeminfo: {:X}
    e_res2: {:?}
    e_lfanew: {:X}
}}"#)}

#[derive(Debug)]
pub struct DOSHeader {
    pub e_magic: u16,       // Magic number
    pub e_cblp: u16,        // Bytes on last page of file
    pub e_cp: u16,          // Pages in file
    pub e_crlc: u16,        // Relocations
    pub e_cparhdr: u16,     // Size of header in paragraphs
    pub e_minalloc: u16,    // Minimum extra paragraphs needed
    pub e_maxalloc: u16,    // Maximum extra paragraphs needed
    pub e_ss: u16,          // Initial (relative) SS value
    pub e_sp: u16,          // Initial SP value
    pub e_csum: u16,        // Checksum
    pub e_ip: u16,          // Initial IP value
    pub e_cs: u16,          // Initial (relative) CS value
    pub e_lfarlc: u16,      // File address of relocation table
    pub e_ovno: u16,        // Overlay number
    pub e_res: [u16; 4],    // Reserved words
    pub e_oemid: u16,       // OEM identifier (for e_oeminfo)
    pub e_oeminfo: u16,     // OEM information; e_oemid specific
    pub e_res2: [u16; 10],  // Reserved words
    pub e_lfanew: i32,      // File address of new exe header
}


impl Display for DOSHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, dos_header_format!(),
            self.e_magic,
            self.e_cblp,
            self.e_cp,
            self.e_crlc,
            self.e_cparhdr,
            self.e_minalloc,
            self.e_maxalloc,
            self.e_ss,
            self.e_sp,
            self.e_csum,
            self.e_ip,
            self.e_cs,
            self.e_lfarlc,
            self.e_ovno,
            self.e_res,
            self.e_oemid,
            self.e_oeminfo,
            self.e_res2,
            self.e_lfanew,
        )
    }
}


impl Parsable for DOSHeader {
    fn from_bytes(data: &Vec<u8>) -> Result<Self, ParseError> {
        let e_magic = take_bytes_start::<u16>(data, 0, 2);
        if !DOSHeader::is_valid_magic(e_magic) {
            return Err(ParseError(String::from("Not a PE file")))
        }

        let e_res_vec = take_words_array_start(data, 28, 2 * 4);
        let e_res2_vec: Vec<u16> = take_words_array_start(data, 40, 2 * 10).try_into().unwrap();

        Ok(DOSHeader{
            e_magic,
            e_cblp: take_bytes_start::<u16>(data, 2, 2),
            e_cp: take_bytes_start::<u16>(data, 4, 2),
            e_crlc: take_bytes_start::<u16>(data, 6, 2),
            e_cparhdr: take_bytes_start::<u16>(data, 8, 2),
            e_minalloc: take_bytes_start::<u16>(data, 10, 2),
            e_maxalloc: take_bytes_start::<u16>(data, 12, 2),
            e_ss: take_bytes_start::<u16>(data, 14, 2),
            e_sp: take_bytes_start::<u16>(data, 16, 2),
            e_csum: take_bytes_start::<u16>(data, 18, 2),
            e_ip: take_bytes_start::<u16>(data, 20, 2),
            e_cs: take_bytes_start::<u16>(data, 22, 2),
            e_lfarlc: take_bytes_start::<u16>(data, 24, 2),
            e_ovno: take_bytes_start::<u16>(data, 26, 2),
            e_res: e_res_vec.try_into().unwrap(),
            e_oemid: take_bytes_start::<u16>(data, 36, 2),
            e_oeminfo: take_bytes_start::<u16>(data, 38, 2),
            e_res2: e_res2_vec.try_into().unwrap(),
            e_lfanew: take_bytes_start::<i32>(data, 60, 4),
        })
    }
}


impl DOSHeader {
    pub fn is_valid_magic(magic: u16) -> bool {
        magic == 0x5a4d
    }
}
