use super::*;

#[derive(Clone, Copy)]
pub enum HAlign {
    Left,
    Centre,
    Right,
}

#[derive(Clone, Copy)]
pub enum VAlign {
    Top,
    Centre,
    Bottom,
}

pub struct Text {
    pub text: String,
    pub halign: HAlign,
    pub valign: VAlign,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            halign: HAlign::Left,
            valign: VAlign::Top,
        }
    }

    pub fn wrap(text: &str) -> Box<Self> {
        Box::new(Self::new(text))
    }
}

impl Widget for Text {
    fn render(&mut self, _context: &mut Rcui, rect: &Rect, _active: bool) {
        let s = self
            .text
            .get(..rect.w.floor() as usize)
            .unwrap_or(&self.text);
        let n = s.len();
        let free_hspace = rect.w - n as f32;
        // TODO(#3): Text does not support wrapping around
        let free_vspace = rect.h - 1.0;

        let x = match self.halign {
            HAlign::Left => rect.x,
            HAlign::Centre => (rect.x + free_hspace * 0.5).floor(),
            HAlign::Right => (rect.x + free_hspace).floor(),
        } as i32;

        let y = match self.valign {
            VAlign::Top => rect.y,
            VAlign::Centre => (rect.y + free_vspace * 0.5).floor(),
            VAlign::Bottom => (rect.y + free_vspace).floor(),
        } as i32;

        mv(y, x);
        addstr(s);
    }

    fn handle_event(&mut self, _context: &mut Rcui, _event: &Event) {}
}
