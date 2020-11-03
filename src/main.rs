use ncurses::*;
use std::panic::{set_hook, take_hook};

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

enum Event {
    Realign(HAlign)
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

    fn handle_event(&mut self, event: &Event) {
        for widget in self.widgets.iter_mut() {
            widget.handle_event(event);
        }
    }
}

#[derive(Clone, Copy)]
enum HAlign {
    Left,
    Centre,
    Right,
}

struct Text {
    text: String,
    halign: HAlign,
}

impl Widget for Text {
    fn render(&self, rect: &Rect) {
        mv(rect.y as i32, rect.x as i32);
        // TODO: Text does not support wrapping around
        let s = self.text.get(..rect.w.floor() as usize).unwrap_or(&self.text);
        let n = s.len();
        let free_space = rect.w - n as f32;
        match self.halign {
            HAlign::Left => {
                addstr(s);
            }
            HAlign::Centre => {
                let padding = (free_space * 0.5).floor() as usize;
                for _ in 0..padding {
                    addstr(" ");
                }
                addstr(s);
                for _ in 0..padding {
                    addstr(" ");
                }
            }
            HAlign::Right => {
                let padding = free_space.floor() as usize;
                for _ in 0..padding {
                    addstr(" ");
                }
                addstr(s);
            }
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Realign(halign) => self.halign = *halign,
        }
    }
}

fn screen_rect() -> Rect {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    getmaxyx(stdscr(), &mut h, &mut w);
    Rect {x: 0.0, y: 0.0, w: w as f32, h: h as f32}
}

fn text(text: &str, halign: HAlign) -> Box<dyn Widget> {
    Box::new(Text { text: text.to_string(), halign })
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

    // TODO: extract this to examples
    let halign = HAlign::Right;
    let mut ui = vbox(vec![
        hbox(vec![ text("hello", halign), text("hello", halign), text("hello", halign) ]),
        hbox(vec![ text("world", halign), text("world", halign), text("world", halign) ]),
        hbox(vec![ text("foo", halign), text("foo", halign), text("foo", halign) ]),
        hbox(vec![ text("bar", halign), text("bar", halign), text("bar", halign) ]),
    ]);

    loop {
        erase();
        ui.render(&screen_rect());
        let key = getch();

        match key as u8 as char {
            'q' => break,
            'z' => ui.handle_event(&Event::Realign(HAlign::Left)),
            'x' => ui.handle_event(&Event::Realign(HAlign::Centre)),
            'c' => ui.handle_event(&Event::Realign(HAlign::Right)),
            _ => {}
        }
    }

    endwin();
}
