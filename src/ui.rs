use crate::models::HexData;

pub fn render(data: &HexData, cursor_pos: usize) {
    println!("Файл: {}", data.filename);
    println!("Размер: {} байт", data.data.len());
    println!("Курсор на позиции: {}", cursor_pos);

    for (i, byte) in data.data.iter().take(16).enumerate() {
        print!("{:02X} ", byte);
        if (i + 1) % 8 == 0 {
            print!(" ");
        }
    }
    println!();
}

pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
