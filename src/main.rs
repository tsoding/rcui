use ncurses::*;
use std::panic::{set_hook, take_hook};

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

enum Event {
}

trait Widget {
    fn render(&self, rect: &Rect);
    fn handle_event(&mut self, event: &Event);
}

struct HBox {
    widgets: Vec<Box<dyn Widget>>
}

impl Widget for HBox {
    fn render(&self, rect: &Rect) {
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

struct VBox {
    widgets: Vec<Box<dyn Widget>>,
}

impl Widget for VBox {
    fn render(&self, rect: &Rect) {
        let n = self.widgets.len();
        let widget_h = rect.h / n as f32;
        for i in 0..n {
            self.widgets[i].render(&Rect {
                x: rect.x,
                y: rect.y + widget_h * i as f32,
                w: rect.w,
                h: widget_h,
            })
        }
    }

    fn handle_event(&mut self, _event: &Event) {
    }
}

struct Text {
    text: String
}

impl Widget for Text {
    fn render(&self, rect: &Rect) {
        mv(rect.y as i32, rect.x as i32);
        // TODO: Text does not support wrapping around
        addstr(self.text.get(..rect.w.floor() as usize).unwrap_or(&self.text));
    }

    fn handle_event(&mut self, _event: &Event) {
    }
}

fn screen_rect() -> Rect {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    getmaxyx(stdscr(), &mut h, &mut w);
    Rect {x: 0.0, y: 0.0, w: w as f32, h: h as f32}
}

fn text(text: &str) -> Box<dyn Widget> {
    Box::new(Text { text: text.to_string() })
}

fn hbox(widgets: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
    Box::new(HBox { widgets })
}

fn vbox(widgets: Vec<Box<dyn Widget>>) -> Box<dyn Widget> {
    Box::new(VBox { widgets })
}

fn main() {
    initscr();

    set_hook(Box::new({
        let default_hook = take_hook();
        move |payload| {
            endwin();
            default_hook(payload);
        }
    }));

    let ui = vbox(vec![
        hbox(vec![ text("hello"), text("hello"), text("hello") ]),
        hbox(vec![ text("world"), text("world"), text("world") ]),
        hbox(vec![ text("foo"), text("foo"), text("foo") ]),
        hbox(vec![ text("bar"), text("bar"), text("bar") ]),
    ]);

    loop {
        erase();
        ui.render(&screen_rect());
        let key = getch();
        if key == 'q' as i32 {
            break;
        }
    }

    endwin();
}
