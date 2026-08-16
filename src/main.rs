mod cli_params;
mod logic;
mod models;
mod ui;

use clap::Parser;
use cli_params::Cli;
use crossterm::event::{read, Event, KeyCode};
use logic::{edit_byte, load_file, save_file};
use ui::{get_user_input, render};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::try_parse()?;

    if cli.version {
        println!("hex_editor {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let filename = match cli.file {
        Some(f) => f,
        None => {
            eprintln!("Файл не указан. Используйте --help для справки.");
            std::process::exit(1);
        }
    };

    let mut hex_data: models::HexData = load_file(&filename)?;

    let mut cursor: usize = cli.offset;
    loop {
        render(&hex_data, cursor);
        println!("\nВведите команду: (e <позиция> <байт>), (s) сохранить, (q) выход");
        //let input = get_user_input();

        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Right => {
                    if cursor < hex_data.data.len() - 1 {
                        cursor += 1;
                    }
                }
                KeyCode::Left => {
                    if cursor > 0 {
                        cursor -= 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
