use super::*;
use std::cmp::{max, min, Ordering};
use std::ops::Range;

#[derive(Default)]
struct Cursor {
    position: usize,
    selection_offset: i32,
}

#[derive(Default)]
pub struct EditField {
    text: Vec<char>,
    buffer: Vec<u8>,
    cursor: Cursor,
}

// TODO(#46): EditField does not support multiple lines (newlines)
// TODO(#47): EditField does not have a way to jump one word forward/backward
// TODO(#48): Some sort of clipboard support for EditField

impl EditField {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            buffer: Vec::new(),
            cursor: Cursor {
                position: 0,
                selection_offset: 0,
            },
        }
    }

    pub fn wrap() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn put_selection_to_clipboard(&self, rcui: &mut Rcui) {
        if let Some(selection) = self.selection() {
            if let Some(text) = self.text.get(selection) {
                rcui.put_to_clipboard(text)
            }
        }
    }

    pub fn paste_from_clipboard(&mut self, rcui: &Rcui) {
        self.insert_chars(rcui.get_clipboard())
    }

    fn selection(&self) -> Option<Range<usize>> {
        let selection_border =
            (self.cursor.position as i32 + self.cursor.selection_offset) as usize;

        match self.cursor.selection_offset.cmp(&0) {
            Ordering::Less => {
                let selection_start = max(selection_border, 0);
                Some(selection_start..self.cursor.position)
            }
            Ordering::Equal => None,
            Ordering::Greater => {
                let selection_end = min(selection_border, self.text.len());
                Some(self.cursor.position..selection_end)
            }
        }
    }

    fn delete_selection(&mut self, selection: Range<usize>) {
        self.cursor.position = selection.start;
        self.text.drain(selection);
        self.unselect()
    }

    fn unselect(&mut self) {
        self.cursor.selection_offset = 0
    }

    pub fn left(&mut self) {
        match self.selection() {
            None => {
                if self.cursor.position > 0 {
                    self.cursor.position -= 1;
                }
            }
            Some(selection) => {
                self.cursor.position = selection.start;
                self.unselect();
            }
        }
    }

    pub fn right(&mut self) {
        match self.selection() {
            None => {
                if self.cursor.position < self.text.len() {
                    self.cursor.position += 1;
                }
            }
            Some(selection) => {
                self.cursor.position = selection.end;
                self.unselect();
            }
        }
    }

    pub fn delete_back(&mut self) {
        match self.selection() {
            None => {
                if self.cursor.position > 0 {
                    self.left();
                    self.text.remove(self.cursor.position);
                }
            }
            Some(selection) => {
                self.delete_selection(selection);
            }
        }
    }

    pub fn delete_front(&mut self) {
        match self.selection() {
            None => {
                if self.cursor.position < self.text.len() {
                    self.text.remove(self.cursor.position);
                }
            }
            Some(selection) => {
                self.delete_selection(selection);
            }
        }
    }

    pub fn insert_chars(&mut self, cs: &[char]) {
        match self.selection() {
            None => {}
            Some(selection) => self.delete_selection(selection),
        }

        if self.cursor.position >= self.text.len() {
            self.text.extend_from_slice(cs);
            self.cursor.position += cs.len();
        } else {
            for c in cs.iter() {
                self.text.insert(self.cursor.position, *c);
                self.cursor.position += 1;
            }
        }
        self.unselect()
    }

    pub fn select_left(&mut self) {
        if (self.cursor.position as i32 + self.cursor.selection_offset) > 0 {
            self.cursor.selection_offset -= 1;
        }
    }

    pub fn select_right(&mut self) {
        if (self.cursor.position as i32 + self.cursor.selection_offset) < self.text.len() as i32 {
            self.cursor.selection_offset += 1;
        }
    }
}

// TODO(#46): EditField does not support multiple lines (newlines)
// TODO(#47): EditField does not have a way to jump one word forward/backward

impl Widget for EditField {
    fn render(&mut self, _context: &mut Rcui, rect: &Rect, active: bool) {
        let x = rect.x.floor() as i32;
        let y = rect.y.floor() as i32;
        mv(y, x);
        // TODO(#35): EditField does not wrap during the rendering
        addstr(&self.text.iter().collect::<String>());
        if active {
            match self.selection() {
                None => {
                    mv(y, x + self.cursor.position as i32);
                    attron(COLOR_PAIR(style::CURSOR_PAIR));
                    if self.cursor.position >= self.text.len() {
                        addstr(" ");
                    } else {
                        addstr(&self.text[self.cursor.position].to_string());
                    }
                    attroff(COLOR_PAIR(style::CURSOR_PAIR));
                }
                Some(selection) => {
                    for position in selection {
                        mv(y, x + position as i32);
                        attron(COLOR_PAIR(style::SELECTION_PAIR));
                        if position >= self.text.len() {
                            addstr(" ");
                        } else {
                            addstr(&self.text[position].to_string());
                        }
                        attroff(COLOR_PAIR(style::SELECTION_PAIR));
                    }
                }
            }
        }
    }

    fn handle_event(&mut self, _context: &mut Rcui, event: &Event) {
        // TODO(#37): move the utf8 buffer mechanism to the main event loop
        if let Event::KeyStroke(key) = event {
            self.buffer.push(*key as u8);
            match String::from_utf8(self.buffer.clone()) {
                Ok(s) => {
                    self.insert_chars(&s.chars().collect::<Vec<_>>());
                    self.buffer.clear()
                }
                Err(_) => {
                    if self.buffer.len() >= 4 {
                        self.buffer.clear()
                    }
                }
            }
        }
    }
}
