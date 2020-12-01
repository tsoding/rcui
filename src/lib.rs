mod column;
mod dummy;
mod edit_field;
mod group;
mod item_list;
mod proxy;
mod row;
pub mod style;
mod text;

#[cfg(unix)]
use ncurses::CURSOR_VISIBILITY::*;
#[cfg(unix)]
use ncurses::*;
#[cfg(windows)]
use pdcurses::CURSOR_VISIBILITY::*;
#[cfg(windows)]
use pdcurses::*;
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
    pub event_queue: VecDeque<Event>,
}

impl Rcui {
    fn new() -> Self {
        Self {
            event_queue: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    // TODO: no support for nested event loops via Rcui::exec()

    pub fn exec(mut ui: Box<dyn Widget>) {
        let mut context = Self::new();

        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");

        initscr();
        keypad(stdscr(), true);
        timeout(10);

        start_color();
        init_pair(style::REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
        init_pair(style::CURSOR_PAIR, COLOR_BLACK, COLOR_WHITE);
        init_pair(style::INACTIVE_CURSOR_PAIR, COLOR_BLACK, COLOR_CYAN);

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

    pub fn quit(&mut self) {
        self.push_event(Event::Quit);
    }
}
