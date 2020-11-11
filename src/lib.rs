mod edit_field;
mod hbox;
mod item_list;
mod proxy;
pub mod style;
mod text;
mod vbox;

use ncurses::CURSOR_VISIBILITY::*;
use ncurses::*;
use std::panic::{set_hook, take_hook};
use std::sync::atomic::{AtomicBool, Ordering};

pub use self::edit_field::*;
pub use self::hbox::*;
pub use self::item_list::*;
pub use self::proxy::*;
pub use self::text::*;
pub use self::vbox::*;

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

// TODO(#5): focus mechanism
// TODO(#13): event mechanism extension that enables with signaling from the bottom of the UI tree to the top
