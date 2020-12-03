#[cfg(unix)]
pub use ncurses::*;
#[cfg(windows)]
pub use pdcurses::*;
