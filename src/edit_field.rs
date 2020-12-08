use super::*;

#[derive(Default)]
pub struct EditField {
    text: Vec<char>,
    buffer: Vec<u8>,
    cursor: usize,
}

// TODO(#45): EditField does not support selections
// TODO: EditField does not support multiple lines (newlines)
// TODO: EditField does not have a way to jump one word forward/backward
// TODO: Some sort of clipboard support for EditField

impl EditField {
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    pub fn wrap() -> Box<Self> {
        Box::new(Self::new())
    }

    pub fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.cursor < self.text.len() {
            self.cursor += 1;
        }
    }

    pub fn delete_back(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.text.remove(self.cursor);
        }
    }

    pub fn delete_front(&mut self) {
        if self.cursor < self.text.len() {
            self.text.remove(self.cursor);
        }
    }

    pub fn insert_chars(&mut self, cs: &[char]) {
        if self.cursor >= self.text.len() {
            self.text.extend_from_slice(cs);
            self.cursor += cs.len();
        } else {
            for c in cs.iter() {
                self.text.insert(self.cursor, *c);
                self.cursor += 1;
            }
        }
    }
}

impl Widget for EditField {
    fn render(&mut self, _context: &mut Rcui, rect: &Rect, active: bool) {
        let x = rect.x.floor() as i32;
        let y = rect.y.floor() as i32;
        mv(y, x);
        // TODO(#35): EditField does not wrap during the rendering
        addstr(&self.text.iter().collect::<String>());
        if active {
            mv(y, x + self.cursor as i32);
            attron(COLOR_PAIR(style::CURSOR_PAIR));
            if self.cursor >= self.text.len() {
                addstr(" ");
            } else {
                addstr(&self.text[self.cursor].to_string());
            }
            attroff(COLOR_PAIR(style::CURSOR_PAIR));
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
