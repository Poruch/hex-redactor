use crate::models::HexData;

pub struct Screen {
    line_size: u32,
    line_count: u32,
    cursor_pos: usize,
}

impl Screen {
    pub fn new(cursor_pos: usize) -> Self {
        Screen {
            line_size: 16,
            line_count: 4,
            cursor_pos: cursor_pos,
        }
    }
    pub fn move_right(&mut self, data: &HexData) -> bool {
        if self.cursor_pos < data.data.len() - 1 {
            self.cursor_pos += 1;
            return true;
        }
        false
    }
    pub fn move_left(&mut self, _data: &HexData) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            return true;
        }
        false
    }
    pub fn move_up(&mut self, _data: &HexData) {
        let line_size = self.line_size as usize;
        if self.cursor_pos >= line_size {
            self.cursor_pos -= line_size;
        }
    }
    pub fn move_down(&mut self, data: &HexData) {
        let line_size = self.line_size as usize;
        let max_pos = data.data.len().saturating_sub(1);
        if self.cursor_pos + line_size <= max_pos {
            self.cursor_pos += line_size;
        }
    }

    pub fn render(&mut self, data: &HexData) {
        println!("Файл: {}", data.filename);
        println!("Размер: {} байт", data.data.len());
        println!("Курсор на позиции: {}", self.cursor_pos);

        let line_size = self.line_size as usize;
        let line_count: usize = self.line_count as usize;

        let page_start = (self.cursor_pos / (line_size * line_count)) * (line_size * line_count);
        let page_end = (page_start + line_size * self.line_count as usize).min(data.data.len());

        // Цвета (ANSI)
        const RESET: &str = "\x1b[0m";
        const HIGHLIGHT: &str = "\x1b[44m\x1b[37m";
        const NORMAL: &str = "\x1b[0m";

        // Проходим по строкам
        let mut row_start = page_start;

        while row_start < page_end {
            // Печатаем адрес строки (смещение в hex)
            print!("{:08X}: ", row_start);

            // Выводим байты в текущей строке
            let row_end = (row_start + line_size).min(data.data.len());
            for (i, byte) in data.data[row_start..row_end].iter().enumerate() {
                let abs_pos = row_start + i;
                if abs_pos == self.cursor_pos {
                    print!("{}{:02X}{} ", HIGHLIGHT, byte, RESET);
                } else {
                    print!("{}{:02X}{} ", NORMAL, byte, NORMAL);
                }

                if (i + 1) % 8 == 0 && i + 1 < line_size {
                    print!(" ");
                }
            }

            if row_end - row_start < line_size {
                for _ in 0..(line_size - (row_end - row_start)) {
                    print!("   ");
                }
            }

            print!(" |");
            for &byte in &data.data[row_start..row_end] {
                let ch = if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
                    byte as char
                } else {
                    '.'
                };
                print!("{}", ch);
            }
            println!();

            row_start += line_size;
        }
    }
}
