pub trait Parsable: Sized {
    fn from_bytes(data: &Vec<u8>) -> Result<Self, ParseError>;
}
#[derive(Debug)]
pub struct ParseError(pub String);