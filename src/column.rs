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
        let cell_size = self.group.cell_size(rect.h);
        let mut y = rect.y;
        for i in 0..n {
            let widget_size = self.group.cells[i].size(cell_size);
            self.group.cells[i].get_widget_mut().render(
                context,
                &Rect {
                    x: rect.x,
                    y,
                    w: rect.w,
                    h: widget_size,
                },
                active && i == self.group.focus,
            );
            y += widget_size;
        }
    }

    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        self.group.handle_event(context, event);
    }
}
