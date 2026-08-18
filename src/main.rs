mod cli_params;
mod logic;
mod models;
mod ui;

use crossterm::event::poll;
use std::{fs::TryLockError::Error, time::Duration};

use clap::Parser;
use cli_params::Cli;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};
use logic::{edit_byte, load_file, save_file};
use ui::Screen;

use models::{HexData, Message, Mode};

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

    let mut hex_data: HexData = load_file(&filename)?;
    let mut messages: Vec<Message> = Vec::new();
    let mut screen = Screen::new(cli.offset)?;
    let mut mode = Mode::View;

    screen.setup()?;
    loop {
        screen.render(&hex_data, &messages, &mode)?;

        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(event) => {
                    let is_global = match (event.modifiers, event.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                            // Ctrl+S: сохранить
                            if event.is_press() {
                                save_file(&hex_data)?;
                                messages.push(Message::new("Сохранение прошло успешно", 3));
                                true
                            } else {
                                false
                            }
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
                        Mode::View => match event.code {
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
                            KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Enter => {
                                if event.is_press() {
                                    mode = Mode::Edit {
                                        input: "".to_string(),
                                    };
                                }
                            }
                            KeyCode::Esc => {
                                break;
                            }
                            _ => {}
                        },
                        Mode::Edit { ref mut input } => match event.code {
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
                                        messages.push(Message::new("Байт успешно изменен", 3));
                                    }
                                    mode = Mode::View;
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
        messages.retain(|msg| !msg.is_expired());
    }
    screen.dispose()?;
    Ok(())
}
