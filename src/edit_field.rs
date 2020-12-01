// TODO(#10): EditField is not implemented

use super::*;

pub struct EditField {
    text: String,
    buffer: Vec<u8>,
    cursor: usize,
}

impl EditField {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            buffer: Vec::new(),
            cursor: 0
        }
    }

    pub fn wrap() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Widget for EditField {
    fn render(&mut self, _context: &mut Rcui, rect: &Rect, _active: bool) {
        let x = rect.x.floor() as i32;
        let y = rect.y.floor() as i32;
        mv(y, x);
        addstr(&self.text);
    }

    fn handle_event(&mut self, _context: &mut Rcui, event: &Event) {
        // TODO: move the utf8 buffer mechanism to the main event loop
        if let Event::KeyStroke(key) = event {
            self.buffer.push(*key as u8);
            match std::str::from_utf8(&self.buffer) {
                Ok(s) => {
                    self.text.push_str(&s);
                    self.buffer.clear()
                }
                Err(_) => if self.buffer.len() >= 4 {
                    self.buffer.clear()
                }
            }
        }
    }
}
