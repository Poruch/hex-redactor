mod cli_params;
mod logic;
mod models;
mod ui;

use clap::Parser;
use cli_params::Cli;
use crossterm::event::{read, Event, KeyCode};
use logic::{edit_byte, load_file, save_file};
use ui::Screen;

use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::stdout;

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

    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut screen = Screen::new(cli.offset);
    let mut hex_data: models::HexData = load_file(&filename)?;

    loop {
        execute!(stdout(), Clear(ClearType::All))?;
        screen.render(&hex_data);
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Right => {
                    if event.is_press() {
                        screen.move_right(&hex_data);
                    }
                }
                KeyCode::Left => {
                    if event.is_press() {
                        screen.move_left(&hex_data);
                    }
                }
                KeyCode::Up => {
                    if event.is_press() {
                        screen.move_up(&hex_data);
                    }
                }
                KeyCode::Down => {
                    if event.is_press() {
                        screen.move_down(&hex_data);
                    }
                }
                KeyCode::Esc => {
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
