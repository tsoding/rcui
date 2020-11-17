use super::*;

pub struct VBox {
    pub widgets: Vec<Box<dyn Widget>>,
}

impl VBox {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self { widgets }
    }

    pub fn wrap(widgets: Vec<Box<dyn Widget>>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }
}

impl Widget for VBox {
    fn render(&mut self, rect: &Rect, active: bool) {
        let n = self.widgets.len();
        let widget_h = rect.h / n as f32;
        for i in 0..n {
            self.widgets[i].render(&Rect {
                x: rect.x,
                y: rect.y + widget_h * i as f32,
                w: rect.w,
                h: widget_h,
            }, active)
        }
    }

    fn handle_event(&mut self, event: &Event) {
        for widget in self.widgets.iter_mut() {
            widget.handle_event(event);
        }
    }
}
