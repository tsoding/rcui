use super::*;

pub struct Proxy<T> {
    pub root: T,
    pub handler: fn(&mut T, &Event),
}

impl<T: Widget> Proxy<T> {
    pub fn wrap(handler: fn(&mut T, &Event), root: T) -> Box<Self> {
        Box::new(Self { root, handler })
    }
}

impl<T: Widget> Widget for Proxy<T> {
    fn render(&mut self, rect: &Rect) {
        self.root.render(rect);
    }

    fn handle_event(&mut self, event: &Event) {
        (self.handler)(&mut self.root, event);
    }
}
