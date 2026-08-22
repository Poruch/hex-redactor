use crate::cli_params::Cli;
use crate::logger::Logger;
use crate::models::{HexData, Mode};
use crate::ui::Screen;

use std::cell::RefCell;
use std::rc::Rc;
pub struct App {
    pub hex_data: HexData,
    pub logger: Rc<RefCell<Logger>>,
    pub screen: Screen,
    pub mode: Mode,
}

impl App {
    pub fn new(filename: &String, cursor_pos: usize) -> Result<App, std::io::Error> {
        let logger = Rc::new(RefCell::new(Logger::new()));
        let screen = Screen::new(Rc::clone(&logger), cursor_pos)?;
        return Ok(App {
            hex_data: HexData::from_file(filename)?,
            logger: logger,
            screen: screen,
            mode: Mode::View,
        });
    }

    pub fn render(&mut self) -> Result<(), std::io::Error> {
        self.screen.render(&self.hex_data, &self.mode)?;
        Ok(())
    }
    pub fn dispose(&self) -> Result<(), std::io::Error> {
        self.screen.dispose()?;
        Ok(())
    }
}
