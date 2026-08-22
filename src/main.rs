mod app;
mod buffer;
mod cli_params;
mod logger;
mod models;
mod ui;

use crossterm::event::poll;
use std::time::Duration;

use clap::Parser;
use cli_params::Cli;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};

use buffer::{copy_selection, paste_clipboard};

use models::{HexData, Message, Mode};

use crate::{app::App, logger::Logger, models::Selection};
use std::rc::Rc;
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
    let mut app = App::new(&filename, cli.offset)?;

    app.screen.setup()?;
    loop {
        app.render();

        if poll(Duration::from_millis(50))? {
            match read()? {
                Event::Key(event) => {
                    app.logger.borrow_mut().add_debug_log(
                        &format!(
                            "Key: code={:?}, modifiers={:?}",
                            event.code, event.modifiers
                        ),
                        3,
                    );
                    let is_global = match (event.modifiers, event.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                            // Ctrl+S: сохранить
                            if event.is_press() {
                                app.hex_data.save_to_file()?;
                                app.logger
                                    .borrow_mut()
                                    .add_user_log("Сохранение прошло успешно", 3);
                                true
                            } else {
                                false
                            }
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                            // Ctrl+Q: выход
                            return Ok(());
                        }
                        (KeyModifiers::CONTROL, KeyCode::Char('c'))
                        | (KeyModifiers::NONE, KeyCode::Char('\x03')) => {
                            if event.is_press() {
                                if let Some((s, e)) = app.screen.selection_range() {
                                    app.logger
                                        .borrow_mut()
                                        .add_system_log(&format!("start copy:{} end:{}", s, e), 5);
                                    copy_selection(
                                        &Selection {
                                            start: Some(s),
                                            end: Some(e),
                                        },
                                        &app.hex_data.data,
                                    )?;
                                }
                                true
                            } else {
                                false
                            }
                        }
                        // Ctrl+V – вставка (включая управляющий символ 0x16)
                        (KeyModifiers::CONTROL, KeyCode::Char('v'))
                        | (KeyModifiers::NONE, KeyCode::Char('\x16')) => {
                            if event.is_press() {
                                app.logger.borrow_mut().add_debug_log("Ctrl+V (вставка)", 3);
                                // Получаем выделение (если есть)
                                let selection = app.screen.selection_range().map_or_else(
                                    || Selection {
                                        start: None,
                                        end: None,
                                    },
                                    |(s, e)| Selection {
                                        start: Some(s),
                                        end: Some(e),
                                    },
                                );
                                paste_clipboard(
                                    &selection,
                                    app.screen.get_pos(),
                                    &mut app.hex_data.data,
                                    &mut app.logger,
                                )?;
                                true
                            } else {
                                false
                            }
                        }
                        _ => false,
                    };
                    if is_global {
                        continue;
                    }
                    match app.mode {
                        Mode::View => match (event.modifiers, event.code) {
                            (KeyModifiers::SHIFT, KeyCode::Right) => {
                                if event.is_press() {
                                    app.screen.move_selection_right(&app.hex_data);
                                }
                            }
                            (KeyModifiers::SHIFT, KeyCode::Left) => {
                                if event.is_press() {
                                    app.screen.move_selection_left(&app.hex_data);
                                }
                            }
                            (_, KeyCode::Right) => {
                                if event.is_press() {
                                    app.screen.move_right(&app.hex_data);
                                }
                            }
                            (_, KeyCode::Left) => {
                                if event.is_press() {
                                    app.screen.move_left(&app.hex_data);
                                }
                            }
                            (_, KeyCode::Up) => {
                                if event.is_press() {
                                    app.screen.move_up(&app.hex_data);
                                }
                            }
                            (_, KeyCode::Down) => {
                                if event.is_press() {
                                    app.screen.move_down(&app.hex_data);
                                }
                            }
                            (_, KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Enter) => {
                                if event.is_press() {
                                    app.mode = Mode::Edit {
                                        input: "".to_string(),
                                    };
                                }
                            }
                            (_, KeyCode::Esc) => {
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
                                        app.hex_data.edit_byte(*app.screen.get_pos(), byte);
                                        app.logger
                                            .borrow_mut()
                                            .add_user_log("Байт успешно изменен", 3);
                                    }
                                    app.mode = Mode::View;
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
        app.logger.borrow_mut().retain()
    }
    app.dispose()?;
    Ok(())
}
