use crate::models::Selection;
use arboard::Clipboard;
use std::cell::RefCell;
use std::rc::Rc;
pub fn copy_selection(selection: &Selection, data: &[u8]) -> Result<(), arboard::Error> {
    if let Some((l, r)) = selection.range() {
        let bytes = &data[l..r];
        // Формируем строку: каждый байт в виде двух шестнадцатеричных цифр
        let hex_string: String = bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");
        let mut clipboard = Clipboard::new()?;
        clipboard.set_text(hex_string)?;
    }
    Ok(())
}

use crate::logger::Logger;

pub fn paste_clipboard(
    selection: &Selection,
    cursor_pos: &mut usize,
    data: &mut Vec<u8>,
    logger: &mut Rc<RefCell<Logger>>,
) -> Result<(), arboard::Error> {
    let mut clipboard = Clipboard::new()?;
    let mut logger = logger.borrow_mut();
    let text = match clipboard.get_text() {
        Ok(t) => t,
        Err(e) => {
            logger.add_system_log(&format!("Ошибка чтения буфера обмена: {}", e), 3);
            return Ok(());
        }
    };
    logger.add_debug_log(&format!("Текст из буфера: {:?}", text), 3);

    let paste_bytes = parse_paste_text(&text);
    logger.add_debug_log(&format!("Парсинг дал {} байт", paste_bytes.len()), 3);
    if paste_bytes.is_empty() {
        logger.add_system_log("Нет данных для вставки", 3);
        return Ok(());
    }

    let pos = *cursor_pos;
    if let Some((l, r)) = selection.range() {
        if l > data.len() || r > data.len() {
            logger.add_system_log(
                &format!(
                    "Индексы выходят за пределы: l={}, r={}, len={}",
                    l,
                    r,
                    data.len()
                ),
                3,
            );
            return Ok(());
        }
        data.splice(l..r, paste_bytes.iter().cloned());
        *cursor_pos = l + paste_bytes.len();
        logger.add_user_log(
            &format!("Вставлено {} байт (замена выделения)", paste_bytes.len()),
            3,
        );
    } else {
        if pos > data.len() {
            logger.add_system_log(
                &format!("cursor_pos {} больше длины данных {}", pos, data.len()),
                3,
            );
            return Ok(());
        }
        data.splice(pos..pos, paste_bytes.iter().cloned());
        *cursor_pos = pos + paste_bytes.len();
        logger.add_user_log(
            &format!("Вставлено {} байт (в позиции курсора)", paste_bytes.len()),
            3,
        );
    }
    Ok(())
}

fn parse_paste_text(text: &str) -> Vec<u8> {
    let trimmed = text.trim();
    let is_hex = trimmed
        .chars()
        .all(|c| c.is_ascii_hexdigit() || c.is_whitespace());
    if is_hex {
        trimmed
            .split_whitespace()
            .filter_map(|s| u8::from_str_radix(s, 16).ok())
            .collect()
    } else {
        trimmed.bytes().collect()
    }
}
