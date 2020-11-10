use super::*;

pub struct HBox {
    pub widgets: Vec<Box<dyn Widget>>,
}

impl HBox {
    pub fn new(widgets: Vec<Box<dyn Widget>>) -> Self {
        Self { widgets }
    }

    pub fn wrap(widgets: Vec<Box<dyn Widget>>) -> Box<Self> {
        Box::new(Self::new(widgets))
    }
}

impl Widget for HBox {
    fn render(&mut self, rect: &Rect) {
        let n = self.widgets.len();
        let widget_w = rect.w / n as f32;
        for i in 0..n {
            self.widgets[i].render(&Rect {
                x: rect.x + widget_w * i as f32,
                y: rect.y,
                w: widget_w,
                h: rect.h,
            })
        }
    }

    fn handle_event(&mut self, event: &Event) {
        for widget in self.widgets.iter_mut() {
            widget.handle_event(event);
        }
    }
}
