mod logic;
mod models;
mod ui;

use logic::{edit_byte, load_file, save_file};
use ui::{get_user_input, render};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "D:\\Repositories\\Pet-projects\\hex-redactor\\text.txt";
    let mut hex_data = load_file(filename)?;

    let mut cursor = 0;
    loop {
        render(&hex_data, cursor);
        println!("\nВведите команду: (e <позиция> <байт>), (s) сохранить, (q) выход");
        let input = get_user_input();

        match input.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["e", pos, byte] => {
                let pos: usize = pos.parse().unwrap_or(0);
                let byte: u8 = byte.parse().unwrap_or(0);
                if edit_byte(&mut hex_data, pos, byte) {
                    println!("Байт изменён.");
                } else {
                    println!("Ошибка: позиция вне диапазона.");
                }
            }
            ["s"] => {
                save_file(&hex_data)?;
                println!("Сохранено.");
            }
            ["q"] => break,
            _ => println!("Неизвестная команда."),
        }
    }

    Ok(())
}
