pub struct COFFFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub timedate_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_jeader: u16,
    pub characteristics: u16,
}
