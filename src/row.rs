use super::*;

pub struct Row {
    pub group: Group,
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Self {
            group: Group::new(cells),
        }
    }

    pub fn wrap(widgets: Vec<Cell>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }

    pub fn focus_next(&mut self) {
        self.group.focus_next();
    }

    pub fn focus_prev(&mut self) {
        self.group.focus_prev();
    }
}

impl Widget for Row {
    fn render(&mut self, context: &mut Rcui, rect: &Rect, active: bool) {
        let n = self.group.cells.len();
        let widget_w = rect.w / n as f32;
        for i in 0..n {
            self.group.cells[i].get_widget_mut().render(
                context,
                &Rect {
                    x: rect.x + widget_w * i as f32,
                    y: rect.y,
                    w: widget_w,
                    h: rect.h,
                },
                active && i == self.group.focus,
            )
        }
    }

    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        self.group.handle_event(context, event);
    }
}
