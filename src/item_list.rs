use super::*;
pub struct Window {
    pub offset: usize,
    pub height: usize
}

pub struct ItemList<T> {
    pub items: Vec<T>,
    pub cursor: usize,
    pub window: Window,
}

impl<T: ToString + Clone> ItemList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            cursor: 0,
            window: Window {
                offset: 0,
                height: 0,
            }
        }
    }

    pub fn wrap(items: Vec<T>) -> Box<Self> {
        Box::new(Self::new(items))
    }

    pub fn up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    // TODO: Define and extract how many lines page-down / page=-up jump
    pub fn page_up(&mut self) {
        for _ in 0..self.window.height {
            self.up();

            if self.cursor == 0 {
                break;
            }
        }
    }

    pub fn down(&mut self) {
        let n = self.items.len();
        if n > 0 && self.cursor < n - 1 {
            self.cursor += 1;
        }
    }

    pub fn page_down(&mut self) {
        for _ in 0..self.window.height {
            self.down();

            if self.cursor == self.items.len() - 1 {
                break;
            }
        }
    }

    pub fn sync_window(&mut self, h: usize) {
        self.window.height = h;

        if self.cursor >= self.window.offset + h {
            self.window.offset = self.cursor - h + 1;
        } else if self.cursor < self.window.offset {
            self.window.offset = self.cursor;
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item)
    }

    pub fn remove(&mut self) -> Option<T> {
        if !self.items.is_empty() {
            let item = self.items.remove(self.cursor);

            if !self.items.is_empty() && self.cursor >= self.items.len() {
                self.cursor = self.items.len() - 1;
            }

            Some(item)
        } else {
            None
        }
    }

    // TODO(#8): Operations to insert new items into the ItemList
    // TODO(#9): Operations to remove items from ItemList
}

impl<T: ToString + Clone> Widget for ItemList<T> {
    fn render(&mut self, _context: &mut Rcui, rect: &Rect, active: bool) {
        let h = rect.h.floor() as usize;
        if h > 0 {
            self.sync_window(h);
            for i in 0..h {
                if self.window.offset + i < self.items.len() {
                    let selected = i + self.window.offset == self.cursor;
                    let color_pair = if selected {
                        if active {
                            style::CURSOR_PAIR
                        } else {
                            style::INACTIVE_CURSOR_PAIR
                        }
                    } else {
                        style::REGULAR_PAIR
                    };

                    attron(COLOR_PAIR(color_pair));
                    let x = rect.x.floor() as i32;
                    let y = (rect.y + i as f32).floor() as i32;
                    let w = rect.w.floor() as usize;
                    mv(y, x);
                    let text = self.items[i + self.window.offset].to_string();
                    if text.len() >= w {
                        addstr(text.get(..w).unwrap_or(&text));
                    } else {
                        addstr(&text);
                        for _ in 0..w - text.len() {
                            addstr(" ");
                        }
                    }
                    attroff(COLOR_PAIR(color_pair));
                }
            }
        }
    }
}
