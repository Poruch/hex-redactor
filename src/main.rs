mod cli_params;
mod logic;
mod models;
mod ui;

use clap::Parser;
use cli_params::Cli;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
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
    let mut mode = crate::models::Mode::View;
    loop {
        execute!(stdout(), Clear(ClearType::All))?;
        screen.render(&hex_data, &mode);

        match read()? {
            Event::Key(event) => {
                let is_global = match (event.modifiers, event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                        // Ctrl+S: сохранить
                        save_file(&hex_data)?;
                        true
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                        // Ctrl+Q: выход
                        return Ok(());
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        // Ctrl+C: игнорируем (чтобы не завершать программу)
                        true
                    }
                    _ => false,
                };
                if is_global {
                    continue;
                }
                match mode {
                    models::Mode::View => match event.code {
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
                        KeyCode::Char('e') | KeyCode::Char('E') => {
                            if event.is_press() {
                                mode = models::Mode::Edit {
                                    input: "".to_string(),
                                };
                            }
                        }
                        KeyCode::Esc => {
                            break;
                        }
                        _ => {}
                    },
                    models::Mode::Edit { ref mut input } => match event.code {
                        KeyCode::Backspace => {
                            if event.is_press() {
                                input.pop();
                            }
                        }
                        KeyCode::Char(c) => {
                            if event.is_press() && c.is_ascii_hexdigit() {
                                if input.len() < 2 {
                                    input.push(c);
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if event.is_press() {
                                if let Ok(byte) = u8::from_str_radix(input, 16) {
                                    edit_byte(&mut hex_data, screen.get_pos(), byte);
                                    mode = models::Mode::View;
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
