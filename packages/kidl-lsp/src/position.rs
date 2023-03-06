use lsp_types::Position;
use ropey::Rope;

pub fn position_to_char(buffer: &Rope, pos: Position) -> usize {
    let line_char = buffer.line_to_char(pos.line as usize);
    let line_char_utf16 = buffer.char_to_utf16_cu(line_char);
    let position_utf16 = line_char_utf16 + pos.character as usize;
    buffer.utf16_cu_to_char(position_utf16)
}

pub fn position_to_byte(buffer: &Rope, pos: Position) -> usize {
    buffer.char_to_byte(position_to_char(buffer, pos))
}

pub fn byte_to_position(buffer: &Rope, byte: usize) -> Position {
    let line = buffer.byte_to_line(byte);
    let line_char = buffer.line_to_char(line);
    let line_char_utf16 = buffer.char_to_utf16_cu(line_char);
    let position_char = buffer.byte_to_char(byte);
    let position_char_utf16 = buffer.char_to_utf16_cu(position_char);
    let character_utf16 = position_char_utf16 - line_char_utf16;

    Position {
        line: line as u32,
        character: character_utf16 as u32,
    }
}
