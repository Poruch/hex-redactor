use crate::logger::{self, Logger};
use crate::models::Mode::Edit;
use crate::models::{HexData, Message, Mode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph},
    Frame, Terminal,
};
use std::cell::RefCell;
use std::rc::Rc;
pub struct Screen {
    line_size: u32,
    line_count: u32,
    cursor_pos: usize,
    anchor: Option<usize>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    logger: Rc<RefCell<Logger>>,
}
use std::io::stdout;
impl Screen {
    pub fn new(logger: Rc<RefCell<Logger>>, cursor_pos: usize) -> Result<Self, std::io::Error> {
        Ok(Screen {
            line_size: 16,
            line_count: 4,
            cursor_pos: cursor_pos,
            anchor: None,
            terminal: Terminal::new(CrosstermBackend::new(stdout()))?,
            logger,
        })
    }
    pub fn get_pos(&mut self) -> &mut usize {
        &mut self.cursor_pos
    }
    pub fn move_right(&mut self, data: &HexData) -> bool {
        self.anchor = None;
        if self.cursor_pos < data.data.len() - 1 {
            self.cursor_pos += 1;
            return true;
        }
        false
    }
    pub fn move_left(&mut self, _data: &HexData) -> bool {
        self.anchor = None;
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            return true;
        }
        false
    }
    fn move_cursor_right_raw(&mut self, data: &HexData) -> bool {
        if self.cursor_pos < data.data.len() - 1 {
            self.cursor_pos += 1;
            true
        } else {
            false
        }
    }

    fn move_cursor_left_raw(&mut self, _data: &HexData) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            true
        } else {
            false
        }
    }
    pub fn move_up(&mut self, _data: &HexData) {
        self.anchor = None;
        let line_size = self.line_size as usize;
        if self.cursor_pos >= line_size {
            self.cursor_pos -= line_size;
        }
    }
    pub fn move_down(&mut self, data: &HexData) {
        self.anchor = None;
        let line_size = self.line_size as usize;
        let max_pos = data.data.len().saturating_sub(1);
        if self.cursor_pos + line_size <= max_pos {
            self.cursor_pos += line_size;
        }
    }
    pub fn move_selection_right(&mut self, data: &HexData) {
        if self.anchor.is_none() {
            self.anchor = Some(self.cursor_pos);
        }
        self.move_cursor_right_raw(data);
    }
    pub fn move_selection_left(&mut self, data: &HexData) {
        if self.anchor.is_none() {
            self.anchor = Some(self.cursor_pos);
        }
        self.move_cursor_left_raw(data);
    }
    pub fn selection_range(&self) -> Option<(usize, usize)> {
        if let (Some(anchor), Some(cursor)) = (self.anchor, Some(self.cursor_pos)) {
            let (start, end) = (anchor.min(cursor), anchor.max(cursor));
            if start != end {
                return Some((start, end));
            }
        }
        None
    }
    pub fn is_single_byte(&self) -> bool {
        if let Some((s, e)) = self.selection_range() {
            s + 1 == e // длина 1
        } else {
            false
        }
    }
    pub fn setup(&self) -> Result<(), std::io::Error> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }
    pub fn dispose(&self) -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn _clear(&self) -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    fn create_layout(area: Rect) -> Vec<Rect> {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(outer_layout[0]);
        let vec: Vec<Rect> = outer_layout.to_vec();
        let mut all_rects = vec.clone();
        all_rects.extend(inner_layout.to_vec());
        all_rects
    }
    pub fn render(&mut self, hex_data: &HexData, mode: &Mode) -> Result<(), std::io::Error> {
        let root_area = self.terminal.size()?;

        let layout = Self::create_layout(root_area.into());
        let addr_width = 10;
        let ascii_width = 3;
        let available = ((layout[2].width as usize).saturating_sub(addr_width + ascii_width) as f32
            * 0.75) as usize
            - 2;
        let mut line_size = 0;
        let mut width_used = 0;
        while width_used + 3 <= available {
            line_size += 1;
            width_used += 3;
            if line_size % 8 == 0 && width_used + 1 <= available {
                width_used += 1;
            }
        }
        if line_size < 1 {
            line_size = 1;
        }
        self.line_size = line_size.min(64);
        self.line_count = (layout[0].height as u32).saturating_sub(2);

        // if let Some((s, e)) = self.selection_range() {
        //     self.logger
        //         .borrow_mut()
        //         .add_debug_log(&format!("Selection range: {}..{}", s, e), 3);
        // } else {
        //     self.logger.borrow_mut().add_debug_log("No selection", 3);
        // }

        let cursor_pos = self.cursor_pos;
        let line_size = self.line_size;
        let line_count = self.line_count;
        let selection = self.selection_range();
        let logger_guard = self.logger.borrow();
        let messages = &logger_guard.get_messages();
        self.terminal.draw(|frame| {
            let layout = Self::create_layout(frame.area());
            Self::render_hex_panel(
                frame,
                hex_data,
                mode,
                layout[2],
                cursor_pos,
                line_size,
                line_count,
                selection,
                Rc::new(None),
            );
            Self::render_status_panel(frame, hex_data, mode, layout[1], cursor_pos);
            Self::render_notification_panel(frame, messages, layout[3])
        })?;
        Ok(())
    }

    fn render_hex_panel(
        frame: &mut Frame,
        data: &HexData,
        mode: &Mode, // пока не используется, но оставим для будущего
        area: Rect,
        cursor_pos: usize,
        line_size: u32,
        line_count: u32,
        selection: Option<(usize, usize)>,
        logger: Rc<Option<Logger>>,
    ) {
        let line_size = line_size as usize;
        let line_count = line_count as usize;

        let page_start = (cursor_pos / (line_size * line_count)) * (line_size * line_count);
        let page_end = (page_start + line_size * line_count).min(data.data.len());

        let mut lines = Vec::new();
        let mut row_start = page_start;

        while row_start < page_end {
            let row_end = (row_start + line_size).min(data.data.len());
            let mut spans = Vec::new();

            spans.push(Span::raw(format!("{:08X}: ", row_start)));

            for (i, byte) in data.data[row_start..row_end].iter().enumerate() {
                let abs_pos = row_start + i;
                let hex_str = format!("{:02X}", byte);
                let span = if let Some((s, e)) = selection {
                    if abs_pos <= e && abs_pos >= s {
                        Span::styled(
                            hex_str,
                            Style::default()
                                .bg(Color::Blue)
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        Span::raw(hex_str)
                    }
                } else {
                    if abs_pos == cursor_pos {
                        match mode {
                            Edit { input } => {
                                let display = if input.is_empty() {
                                    "__".to_string()
                                } else if input.len() == 1 {
                                    format!("_{}", input) // ведущий ноль
                                } else {
                                    input.clone()
                                };
                                Span::styled(
                                    display,
                                    Style::default()
                                        .bg(Color::Blue)
                                        .fg(Color::White)
                                        .add_modifier(Modifier::BOLD),
                                )
                            }
                            Mode::View => Span::styled(
                                hex_str,
                                Style::default()
                                    .bg(Color::Blue)
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            _ => Span::raw(hex_str),
                        }
                    } else {
                        Span::raw(hex_str)
                    }
                };
                spans.push(span);
                spans.push(Span::raw(" "));

                // Добавляем дополнительный пробел после 8-го байта
                if (i + 1) % 8 == 0 && i + 1 < line_size {
                    spans.push(Span::raw(" "));
                }
            }

            if row_end - row_start < line_size {
                for _ in 0..(line_size - (row_end - row_start)) {
                    spans.push(Span::raw("   "));
                }
            }

            spans.push(Span::raw(" | "));
            for (i, byte) in data.data[row_start..row_end].iter().enumerate() {
                let abs_pos = row_start + i;
                let ch = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                    *byte as char
                } else {
                    '.'
                };
                if abs_pos == cursor_pos {
                    spans.push(Span::styled(
                        ch.to_string(),
                        Style::default()
                            .bg(Color::Blue)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::raw(ch.to_string()));
                }
            }

            lines.push(Line::from(spans));
            row_start += line_size;
        }

        let list = List::new(lines).block(Block::default().borders(Borders::ALL).title("Hex Dump"));
        frame.render_widget(list, area);
    }
    fn render_status_panel(
        frame: &mut Frame,
        data: &HexData,
        mode: &Mode,
        area: Rect,
        cursor_pos: usize,
    ) {
        let status_text = format!(
            "Файл: {} | Размер: {} байт | Курсор: {} | Режим: {:?}",
            data.filename,
            data.data.len(),
            cursor_pos,
            mode
        );

        let paragraph = Paragraph::new(status_text)
            .block(Block::default().borders(Borders::ALL).title("Status"));
        frame.render_widget(paragraph, area);
    }

    fn render_notification_panel(frame: &mut Frame, messages: &[Message], area: Rect) {
        let mut msgs = messages
            .iter()
            .map(|msg| format!("[{}]", msg.text))
            .collect::<Vec<String>>();

        msgs.reverse();
        let notification_text: String = msgs.join("\n");

        let paragraph = Paragraph::new(notification_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Notifications"),
        );
        frame.render_widget(paragraph, area);
    }
}
