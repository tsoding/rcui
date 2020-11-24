use super::*;

pub struct Column {
    pub group: Group,
}

impl Column {
    pub fn new(widgets: Vec<Cell>) -> Self {
        Self {
            group: Group::new(widgets),
        }
    }

    pub fn wrap(widgets: Vec<Cell>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }
}

impl Widget for Column {
    fn render(&mut self, context: &mut Rcui, rect: &Rect, active: bool) {
        let n = self.group.cells.len();
        let widget_h = rect.h / n as f32;
        for i in 0..n {
            self.group.cells[i].get_widget_mut().render(
                context,
                &Rect {
                    x: rect.x,
                    y: rect.y + widget_h * i as f32,
                    w: rect.w,
                    h: widget_h,
                },
                active && i == self.group.focus,
            )
        }
    }

    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        self.group.handle_event(context, event);
    }
}
