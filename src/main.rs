use ncurses::*;
use std::panic::{set_hook, take_hook};

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

enum Event {
    Realign(HAlign, VAlign)
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

#[derive(Clone, Copy)]
enum VAlign {
    Top,
    Centre,
    Bottom
}

struct Text {
    text: String,
    halign: HAlign,
    valign: VAlign,
}

impl Widget for Text {
    fn render(&self, rect: &Rect) {
        let s = self.text.get(..rect.w.floor() as usize).unwrap_or(&self.text);
        let n = s.len();
        let free_hspace = rect.w - n as f32;
        // TODO: Text does not support wrapping around
        let free_vspace = rect.h - 1.0;

        match self.valign {
            VAlign::Top => {
                mv(rect.y as i32, rect.x as i32);
            },
            VAlign::Centre => {
                mv((rect.y + free_vspace * 0.5).floor() as i32, rect.x as i32);
            },
            VAlign::Bottom => {
                mv((rect.y + free_vspace).floor() as i32, rect.x as i32);
            },
        }

        match self.halign {
            HAlign::Left => {
                addstr(s);
            }
            HAlign::Centre => {
                let padding = (free_hspace * 0.5).floor() as usize;
                for _ in 0..padding {
                    addstr(" ");
                }
                addstr(s);
                for _ in 0..padding {
                    addstr(" ");
                }
            }
            HAlign::Right => {
                let padding = free_hspace.floor() as usize;
                for _ in 0..padding {
                    addstr(" ");
                }
                addstr(s);
            }
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Realign(halign, valign) => {
                self.halign = *halign;
                self.valign = *valign;
            },
        }
    }
}

fn screen_rect() -> Rect {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    getmaxyx(stdscr(), &mut h, &mut w);
    Rect {x: 0.0, y: 0.0, w: w as f32, h: h as f32}
}

fn text(text: &str) -> Box<dyn Widget> {
    Box::new(Text { text: text.to_string(), halign: HAlign::Left, valign: VAlign::Top })
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
    let mut ui = vbox(vec![
        hbox(vec![ text("hello"), text("hello"), text("hello") ]),
        hbox(vec![ text("world"), text("world"), text("world") ]),
        hbox(vec![ text("foo"), text("foo"), text("foo") ]),
        hbox(vec![ text("bar"), text("bar"), text("bar") ]),
    ]);

    loop {
        erase();
        ui.render(&screen_rect());
        let key = getch();

        match key as u8 as char {
            't' => break,

            'q' => ui.handle_event(&Event::Realign(HAlign::Left, VAlign::Top)),
            'w' => ui.handle_event(&Event::Realign(HAlign::Centre, VAlign::Top)),
            'e' => ui.handle_event(&Event::Realign(HAlign::Right, VAlign::Top)),

            'a' => ui.handle_event(&Event::Realign(HAlign::Left, VAlign::Centre)),
            's' => ui.handle_event(&Event::Realign(HAlign::Centre, VAlign::Centre)),
            'd' => ui.handle_event(&Event::Realign(HAlign::Right, VAlign::Centre)),

            'z' => ui.handle_event(&Event::Realign(HAlign::Left, VAlign::Bottom)),
            'x' => ui.handle_event(&Event::Realign(HAlign::Centre, VAlign::Bottom)),
            'c' => ui.handle_event(&Event::Realign(HAlign::Right, VAlign::Bottom)),
            _ => {}
        }
    }

    endwin();
}
