use ncurses::*;
use std::panic::{set_hook, take_hook};

struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

enum Event {
    KeyStroke(i32),
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
            Event::KeyStroke(key) => {
                match *key as u8 as char {
                    'q' => {
                        self.halign = HAlign::Left;
                        self.valign = VAlign::Top;
                    },

                    'w' => {
                        self.halign = HAlign::Centre;
                        self.valign = VAlign::Top;
                    },

                    'e' => {
                        self.halign = HAlign::Right;
                        self.valign = VAlign::Top;
                    },

                    'a' => {
                        self.halign = HAlign::Left;
                        self.valign = VAlign::Centre;
                    },

                    's' => {
                        self.halign = HAlign::Centre;
                        self.valign = VAlign::Centre;
                    },

                    'd' => {
                        self.halign = HAlign::Right;
                        self.valign = VAlign::Centre;
                    },

                    'z' => {
                        self.halign = HAlign::Left;
                        self.valign = VAlign::Bottom;
                    },

                    'x' => {
                        self.halign = HAlign::Centre;
                        self.valign = VAlign::Bottom;
                    },

                    'c' => {
                        self.halign = HAlign::Right;
                        self.valign = VAlign::Bottom;
                    },

                    _ => {}
                }
            }
        }
    }
}

struct ItemList<T> {
    items: Vec<T>,
}

impl<T: ToString + Clone> Widget for ItemList<T> {
    fn render(&self, rect: &Rect) {
        for (i, item) in self.items.iter().enumerate() {
            let text = Text {
                text: item.to_string(),
                halign: HAlign::Left,
                valign: VAlign::Top
            };
            text.render(&Rect {
                x: rect.x,
                y: rect.y + i as f32,
                w: rect.w,
                h: 1.0,
            });

            if i as f32 >= rect.h {
                break;
            }
        }
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
        hbox(vec![
            Box::new(ItemList {
                items: vec!["item1", "item2", "item3"]
            }),
            text("hello"),
            text("hello"),
        ]),
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
            _ => {
                ui.handle_event(&Event::KeyStroke(key));
            }
        }
    }
    endwin();
}
