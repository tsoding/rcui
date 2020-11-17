use super::*;

pub struct XBox {
    pub widgets: Vec<Box<dyn Widget>>,
    pub focus: usize
}

impl XBox {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self { widgets, focus: 0 }
    }

    pub fn wrap(widgets: Vec<Box<dyn Widget>>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }

    pub fn focus_next(&mut self) {
        if !self.widgets.is_empty() {
            self.focus = (self.focus + 1) % self.widgets.len()
        }
    }

    pub fn focus_prev(&mut self) {
        if !self.widgets.is_empty() {
            if self.focus == 0 {
                self.focus = self.widgets.len() - 1;
            } else {
                self.focus -= 1;
            }
        }
    }
}

impl Widget for XBox {
    fn render(&mut self, _rect: &Rect, _active: bool) {}

    fn handle_event(&mut self, event: &Event) {
        if let Some(widget) = self.widgets.get_mut(self.focus) {
            widget.handle_event(event);
        }
    }
}
