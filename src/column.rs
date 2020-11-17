use super::*;

pub struct Column {
    pub xbox: XBox
}

impl Column {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self { xbox: XBox::new(widgets) }
    }

    pub fn wrap(widgets: Vec<Box<dyn Widget>>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }
}

impl Widget for Column {
    fn render(&mut self, rect: &Rect, active: bool) {
        let n = self.xbox.widgets.len();
        let widget_h = rect.h / n as f32;
        for i in 0..n {
            self.xbox.widgets[i].render(&Rect {
                x: rect.x,
                y: rect.y + widget_h * i as f32,
                w: rect.w,
                h: widget_h,
            }, active && i == self.xbox.focus)
        }
    }

    fn handle_event(&mut self, event: &Event) {
        self.xbox.handle_event(event);
    }
}
