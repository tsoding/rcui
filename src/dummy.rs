use super::*;

#[derive(Default)]
pub struct Dummy;

impl Dummy {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wrap() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl Widget for Dummy {
    fn render(&mut self, _context: &mut Rcui, _rect: &Rect, _active: bool) {}
    fn handle_event(&mut self, _context: &mut Rcui, _event: &Event) {}
}
