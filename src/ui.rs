use crate::models::HexData;

pub fn render(data: &HexData, cursor_pos: usize) {
    println!("Файл: {}", data.filename);
    println!("Размер: {} байт", data.data.len());
    println!("Курсор на позиции: {}", cursor_pos);

    const RESET: &str = "\x1b[0m";
    const HIGHLIGHT: &str = "\x1b[44m\x1b[37m"; // синий фон, белый текст
    const NORMAL: &str = "\x1b[0m";

    for (i, byte) in data.data.iter().take(16).enumerate() {
        let pos = i; // номер байта на текущей строке (0..15)
        let current_byte_pos = i; // если выводим только первую строку, то позиция = i

        // Если текущая позиция равна cursor_pos, применяем подсветку
        if current_byte_pos == cursor_pos {
            print!("{}{:02X}{} ", HIGHLIGHT, byte, RESET);
        } else {
            print!("{}{:02X}{} ", NORMAL, byte, NORMAL);
        }

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
