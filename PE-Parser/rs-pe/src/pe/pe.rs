pub struct PE {
    pub signature: &str
}


impl Default for PE {
    fn default() -> Self {
        PE {
            signature: "MZ"
        }
    }
}
