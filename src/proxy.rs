use super::*;

pub struct Proxy<T> {
    pub origin: T,
    pub handler: fn(&mut T, &Event),
}

impl<T: Widget> Proxy<T> {
    pub fn new(handler: fn(&mut T, &Event), origin: T) -> Self {
        Self { origin, handler }
    }

    pub fn wrap(handler: fn(&mut T, &Event), origin: T) -> Box<Self> {
        Box::new(Self::new(handler, origin))
    }
}

impl<T: Widget> Widget for Proxy<T> {
    fn render(&mut self, rect: &Rect) {
        self.origin.render(rect);
    }

    fn handle_event(&mut self, event: &Event) {
        (self.handler)(&mut self.origin, event);
    }
}
