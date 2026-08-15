use crate::models::HexData;
use std::fs;

pub fn load_file(path: &str) -> Result<HexData, std::io::Error> {
    let data = fs::read(path)?;
    Ok(HexData {
        data,
        filename: path.to_string(),
    })
}

pub fn save_file(data: &HexData) -> Result<(), std::io::Error> {
    fs::write(&data.filename, &data.data)?;
    Ok(())
}

pub fn edit_byte(data: &mut HexData, pos: usize, new_byte: u8) -> bool {
    if pos < data.data.len() {
        data.data[pos] = new_byte;
        true
    } else {
        false
    }
}
