pub struct HexData {
    pub data: Vec<u8>,
    pub filename: String,
}
pub enum Mode {
    View,
    Edit { input: String },
    Command { input: String },
}
