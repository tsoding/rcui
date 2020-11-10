pub mod style;

use ncurses::CURSOR_VISIBILITY::*;
use ncurses::*;
use std::panic::{set_hook, take_hook};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub enum Event {
    KeyStroke(i32),
}

pub trait Widget {
    fn render(&mut self, rect: &Rect);
    fn handle_event(&mut self, event: &Event);
}

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
    fn render(&mut self, rect: &Rect) {
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
pub enum HAlign {
    Left,
    Centre,
    Right,
}

#[derive(Clone, Copy)]
pub enum VAlign {
    Top,
    Centre,
    Bottom,
}

pub struct Text {
    pub text: String,
    pub halign: HAlign,
    pub valign: VAlign,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            halign: HAlign::Left,
            valign: VAlign::Top,
        }
    }

    pub fn wrap(text: &str) -> Box<Self> {
        Box::new(Self::new(text))
    }
}

impl Widget for Text {
    fn render(&mut self, rect: &Rect) {
        let s = self
            .text
            .get(..rect.w.floor() as usize)
            .unwrap_or(&self.text);
        let n = s.len();
        let free_hspace = rect.w - n as f32;
        // TODO(#3): Text does not support wrapping around
        let free_vspace = rect.h - 1.0;

        match self.valign {
            VAlign::Top => {
                mv(rect.y as i32, rect.x as i32);
            }
            VAlign::Centre => {
                mv((rect.y + free_vspace * 0.5).floor() as i32, rect.x as i32);
            }
            VAlign::Bottom => {
                mv((rect.y + free_vspace).floor() as i32, rect.x as i32);
            }
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

    fn handle_event(&mut self, _event: &Event) {}
}

pub struct ItemList<T> {
    pub items: Vec<T>,
    pub cursor: usize,
    pub scroll: usize,
}

impl<T: ToString + Clone> ItemList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            cursor: 0,
            scroll: 0,
        }
    }

    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.cursor < self.items.len() - 1 {
            self.cursor += 1;
        }
    }

    pub fn sync_scroll(&mut self, h: usize) {
        if self.cursor >= self.scroll + h {
            self.scroll = self.cursor - h + 1;
        } else if self.cursor < self.scroll {
            self.scroll = self.cursor;
        }
    }

    // TODO(#8): Operations to insert new items into the ItemList
    // TODO(#9): Operations to remove items from ItemList
}

// TODO: EditField is not implemented

impl<T: ToString + Clone> Widget for ItemList<T> {
    fn render(&mut self, rect: &Rect) {
        let h = rect.h.floor() as usize;
        if h > 0 {
            self.sync_scroll(h);
            for i in 0..h {
                if self.scroll + i < self.items.len() {
                    let mut text = Text {
                        text: self.items[i + self.scroll].to_string(),
                        halign: HAlign::Left,
                        valign: VAlign::Top,
                    };

                    let selected = i + self.scroll == self.cursor;
                    let color_pair = if selected {
                        style::CURSOR_PAIR
                    } else {
                        style::REGULAR_PAIR
                    };

                    attron(COLOR_PAIR(color_pair));
                    text.render(&Rect {
                        x: rect.x,
                        y: rect.y + i as f32,
                        w: rect.w,
                        h: 1.0,
                    });
                    attroff(COLOR_PAIR(color_pair));
                }
            }
        }
    }

    fn handle_event(&mut self, _event: &Event) {}
}

pub fn screen_rect() -> Rect {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    getmaxyx(stdscr(), &mut h, &mut w);
    Rect {
        x: 0.0,
        y: 0.0,
        w: w as f32,
        h: h as f32,
    }
}

static QUIT: AtomicBool = AtomicBool::new(false);

pub fn quit() {
    QUIT.store(true, Ordering::Relaxed);
}

pub fn exec(mut ui: Box<dyn Widget>) {
    initscr();

    start_color();
    init_pair(style::REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(style::CURSOR_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(style::UNFOCUSED_CURSOR_PAIR, COLOR_BLACK, COLOR_CYAN);

    curs_set(CURSOR_INVISIBLE);

    set_hook(Box::new({
        let default_hook = take_hook();
        move |payload| {
            endwin();
            default_hook(payload);
        }
    }));

    while !QUIT.swap(false, Ordering::Relaxed) {
        erase();
        ui.render(&screen_rect());
        let key = getch();
        ui.handle_event(&Event::KeyStroke(key));
    }

    endwin();
}

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

// TODO(#5): focus mechanism
