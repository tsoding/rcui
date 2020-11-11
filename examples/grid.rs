use rcui::*;

fn text_cell(s: &str) -> Box<Text> {
    Box::new(Text {
        text: s.to_string(),
        halign: HAlign::Centre,
        valign: VAlign::Centre,
    })
}

fn main() {
    rcui::exec(Proxy::wrap(
        |origin, event| {
            match event {
                Event::KeyStroke(key) => {
                    if *key as u8 as char == 'q' {
                        rcui::quit();
                    }
                }
            }
            origin.handle_event(event);
        },
        VBox::new(vec![
            HBox::wrap(vec![
                text_cell("hello"),
                text_cell("hello"),
                text_cell("hello"),
            ]),
            HBox::wrap(vec![
                text_cell("world"),
                text_cell("world"),
                text_cell("world"),
            ]),
            HBox::wrap(vec![text_cell("foo"), text_cell("foo"), text_cell("foo")]),
            HBox::wrap(vec![text_cell("bar"), text_cell("bar"), text_cell("bar")]),
        ]),
    ));

    println!("Quitting gracefully uwu");
}
