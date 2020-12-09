mod column;
pub mod curses;
mod dummy;
mod edit_field;
mod group;
mod item_list;
mod proxy;
mod row;
pub mod style;
mod text;

use curses::CURSOR_VISIBILITY::*;
use curses::*;
use std::collections::VecDeque;
use std::panic::{set_hook, take_hook};

pub use self::column::*;
pub use self::dummy::*;
pub use self::edit_field::*;
pub use self::group::*;
pub use self::item_list::*;
pub use self::proxy::*;
pub use self::row::*;
pub use self::text::*;
pub use std::any::Any;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub enum Event {
    Quit,
    KeyStroke(i32),
    Custom(Box<dyn Any>),
}

pub trait Widget {
    fn render(&mut self, _context: &mut Rcui, _rect: &Rect, _active: bool) {}
    fn handle_event(&mut self, _context: &mut Rcui, _event: &Event) {}
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

pub struct Rcui {
    event_queue: VecDeque<Event>,
    clipboard: Vec<char>,
}

impl Rcui {
    fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
            clipboard: Vec::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    // TODO(#36): no support for nested event loops via Rcui::exec()

    pub fn exec(mut ui: Box<dyn Widget>) {
        let mut context = Self::new();

        unsafe {
            libc::setlocale(libc::LC_ALL, "en_US.UTF-8\0".as_ptr().cast());
        }

        initscr();
        keypad(stdscr(), true);
        timeout(10);

        style::init_style();

        curs_set(CURSOR_INVISIBLE);

        set_hook(Box::new({
            let default_hook = take_hook();
            move |payload| {
                endwin();
                default_hook(payload);
            }
        }));

        let mut quit = false;
        while !quit {
            #[cfg(windows)]
            if is_termresized() {
                resize_term(0, 0);
            }
            erase();
            ui.render(&mut context, &screen_rect(), true);

            // Busy waiting on the key event
            let mut key = getch();
            while key == ERR {
                key = getch();
            }

            // Flushing everything we've got
            while key != ERR {
                context.push_event(Event::KeyStroke(key));
                key = getch();
            }

            // Handling all of the events from the queue
            while !context.event_queue.is_empty() {
                if let Some(event) = context.event_queue.pop_front() {
                    if let Event::Quit = event {
                        quit = true;
                    }

                    ui.handle_event(&mut context, &event);
                };
            }
        }

        endwin();
    }

    pub fn put_to_clipboard(&mut self, text: &[char]) {
        self.clipboard.clear();
        self.clipboard.extend_from_slice(text);
    }

    pub fn get_clipboard(&self) -> &[char] {
        &self.clipboard
    }

    pub fn quit(&mut self) {
        self.push_event(Event::Quit);
    }
}
