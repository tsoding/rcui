use ncurses::*;

pub const REGULAR_PAIR: i16 = 1;
pub const CURSOR_PAIR: i16 = 2;
pub const UNFOCUSED_CURSOR_PAIR: i16 = 3;

pub fn init_style() {
    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(CURSOR_PAIR, COLOR_BLACK, COLOR_WHITE);
    init_pair(UNFOCUSED_CURSOR_PAIR, COLOR_BLACK, COLOR_CYAN);
}
