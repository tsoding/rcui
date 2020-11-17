use super::*;

pub struct Row {
    pub xbox: XBox
}

impl Row {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self { xbox: XBox::new(widgets) }
    }

    pub fn wrap(widgets: Vec<Box<dyn Widget>>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }

    pub fn focus_next(&mut self) {
        self.xbox.focus_next();
    }

    pub fn focus_prev(&mut self) {
        self.xbox.focus_prev();
    }
}

impl Widget for Row {
    fn render(&mut self, rect: &Rect, active: bool) {
        let n = self.xbox.widgets.len();
        let widget_w = rect.w / n as f32;
        for i in 0..n {
            self.xbox.widgets[i].render(&Rect {
                x: rect.x + widget_w * i as f32,
                y: rect.y,
                w: widget_w,
                h: rect.h,
            }, active && i == self.xbox.focus)
        }
    }

    fn handle_event(&mut self, event: &Event) {
        self.xbox.handle_event(event);
    }
}
