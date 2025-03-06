use crate::prelude::*;

pub fn read_file(file_name: &str) -> Result<Vec<u8>> {
    Ok(std::fs::read(std::path::Path::new("stuff").join(file_name))?)
}


pub fn stuff_path(file_name: &str) -> Result<String> {
    Ok(std::fs::read_to_string(std::path::Path::new("stuff").join(file_name))?)
}
