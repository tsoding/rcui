use rcui::*;

struct MyText {
    text: Text,
}

impl Widget for MyText {
    fn render(&self, rect: &rcui::Rect) {
        self.text.render(rect)
    }

    fn handle_event(&mut self, event: &rcui::Event) {
        match event {
            Event::KeyStroke(key) => {
                match *key as u8 as char {
                    'q' => {
                        self.text.halign = HAlign::Left;
                        self.text.valign = VAlign::Top;
                    },

                    'w' => {
                        self.text.halign = HAlign::Centre;
                        self.text.valign = VAlign::Top;
                    },

                    'e' => {
                        self.text.halign = HAlign::Right;
                        self.text.valign = VAlign::Top;
                    },

                    'a' => {
                        self.text.halign = HAlign::Left;
                        self.text.valign = VAlign::Centre;
                    },

                    's' => {
                        self.text.halign = HAlign::Centre;
                        self.text.valign = VAlign::Centre;
                    },

                    'd' => {
                        self.text.halign = HAlign::Right;
                        self.text.valign = VAlign::Centre;
                    },

                    'z' => {
                        self.text.halign = HAlign::Left;
                        self.text.valign = VAlign::Bottom;
                    },

                    'x' => {
                        self.text.halign = HAlign::Centre;
                        self.text.valign = VAlign::Bottom;
                    },

                    'c' => {
                        self.text.halign = HAlign::Right;
                        self.text.valign = VAlign::Bottom;
                    },

                    _ => {}
                }
            }
        }
    }
}

pub fn my_text(text: &str) -> Box<dyn Widget> {
    Box::new(MyText {
        text: Text {
            text: text.to_string(),
            halign: HAlign::Left,
            valign: VAlign::Top,
        }
    })
}

struct Quit {
    root: Box<dyn Widget>
}

impl Widget for Quit {
    fn render(&self, rect: &Rect) {
        self.root.render(rect)
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyStroke(key) => {
                match *key as u8 as char {
                    't' => {
                        println!("Quitting gracefully uwu");
                        rcui::quit()
                    }
                    _ => {}
                }
            }
        }

        self.root.handle_event(event);
    }
}

fn main() {
    rcui::exec(
        Box::new(Quit {
            root: vbox(vec![
                hbox(vec![
                    Box::new(ItemList {
                        items: vec!["item1", "item2", "item3"]
                    }),
                    my_text("hello"),
                    my_text("hello"),
                ]),
                hbox(vec![ my_text("world"), my_text("world"), my_text("world") ]),
                hbox(vec![ my_text("foo"), my_text("foo"), my_text("foo") ]),
                hbox(vec![ my_text("bar"), my_text("bar"), my_text("bar") ]),
            ])
        })
    );
}
