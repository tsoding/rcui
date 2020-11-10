use rcui::*;

struct MyText {
    text: Text,
}

impl MyText {
    fn new(text: &str) -> Self {
        Self {
            text: Text::new(text)
        }
    }

    fn wrap(text: &str) -> Box<Self> {
        Box::new(Self::new(text))
    }
}

impl Widget for MyText {
    fn render(&self, rect: &rcui::Rect) {
        self.text.render(rect)
    }

    fn handle_event(&mut self, event: &rcui::Event) {
        match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => {
                    self.text.halign = HAlign::Left;
                    self.text.valign = VAlign::Top;
                }

                'w' => {
                    self.text.halign = HAlign::Centre;
                    self.text.valign = VAlign::Top;
                }

                'e' => {
                    self.text.halign = HAlign::Right;
                    self.text.valign = VAlign::Top;
                }

                'a' => {
                    self.text.halign = HAlign::Left;
                    self.text.valign = VAlign::Centre;
                }

                's' => {
                    self.text.halign = HAlign::Centre;
                    self.text.valign = VAlign::Centre;
                }

                'd' => {
                    self.text.halign = HAlign::Right;
                    self.text.valign = VAlign::Centre;
                }

                'z' => {
                    self.text.halign = HAlign::Left;
                    self.text.valign = VAlign::Bottom;
                }

                'x' => {
                    self.text.halign = HAlign::Centre;
                    self.text.valign = VAlign::Bottom;
                }

                'c' => {
                    self.text.halign = HAlign::Right;
                    self.text.valign = VAlign::Bottom;
                }

                _ => {}
            },
        }
    }
}

fn main() {
    rcui::exec(HardProxy::wrap(
        |root, event| {
            match event {
                Event::KeyStroke(key) => {
                    if *key as u8 as char == 't' {
                        rcui::quit();
                    }
                }
            }
            root.handle_event(event);
        },
        VBox::new(vec![
            HBox::wrap(vec![MyText::wrap("hello"), MyText::wrap("hello"), MyText::wrap("hello")]),
            HBox::wrap(vec![MyText::wrap("world"), MyText::wrap("world"), MyText::wrap("world")]),
            HBox::wrap(vec![MyText::wrap("foo"), MyText::wrap("foo"), MyText::wrap("foo")]),
            HBox::wrap(vec![MyText::wrap("bar"), MyText::wrap("bar"), MyText::wrap("bar")]),
        ]),
    ));

    println!("Quitting gracefully uwu");
}
