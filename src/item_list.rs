use super::*;

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

    pub fn wrap(items: Vec<T>) -> Box<Self> {
        Box::new(Self::new(items))
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
    // TODO(#14): Operations for page-up/page-down for ItemList
}

impl<T: ToString + Clone> Widget for ItemList<T> {
    fn render(&mut self, rect: &Rect, active: bool) {
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
                        if active {
                            style::CURSOR_PAIR
                        } else {
                            style::INACTIVE_CURSOR_PAIR
                        }
                    } else {
                        style::REGULAR_PAIR
                    };

                    attron(COLOR_PAIR(color_pair));
                    text.render(&Rect {
                        x: rect.x,
                        y: rect.y + i as f32,
                        w: rect.w,
                        h: 1.0,
                    }, active);
                    attroff(COLOR_PAIR(color_pair));
                }
            }
        }
    }

    fn handle_event(&mut self, _event: &Event) {}
}
