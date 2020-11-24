use super::*;

pub struct Column {
    pub group: Group
}

impl Column {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self { group: Group::new(widgets) }
    }

    pub fn wrap(widgets: Vec<Box<dyn Widget>>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }
}

impl Widget for Column {
    fn render(&mut self, context: &mut Context, rect: &Rect, active: bool) {
        let n = self.group.widgets.len();
        let widget_h = rect.h / n as f32;
        for i in 0..n {
            self.group.widgets[i].render(context, &Rect {
                x: rect.x,
                y: rect.y + widget_h * i as f32,
                w: rect.w,
                h: widget_h,
            }, active && i == self.group.focus)
        }
    }

    fn handle_event(&mut self, context: &mut Context, event: &Event) {
        self.group.handle_event(context, event);
    }
}
