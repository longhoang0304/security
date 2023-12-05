use crate::pe::headers::parsable::{Parsable, ParseError};

pub struct RichHeader {}


impl Parsable for RichHeader {
    fn from_bytes(data: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(RichHeader{})
    }
}
