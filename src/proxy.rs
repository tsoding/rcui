use super::*;

pub struct Proxy<T> {
    pub origin: T,
    pub handler: fn(&mut T, &mut Rcui, &Event),
}

impl<T: Widget> Proxy<T> {
    pub fn new(handler: fn(&mut T, &mut Rcui, &Event), origin: T) -> Self {
        Self { origin, handler }
    }

    pub fn wrap(handler: fn(&mut T, &mut Rcui, &Event), origin: T) -> Box<Self> {
        Box::new(Self::new(handler, origin))
    }
}

impl<T: Widget> Widget for Proxy<T> {
    fn render(&mut self, context: &mut Rcui, rect: &Rect, active: bool) {
        self.origin.render(context, rect, active);
    }

    fn handle_event(&mut self, context: &mut Rcui, event: &Event) {
        (self.handler)(&mut self.origin, context, event);
    }
}
