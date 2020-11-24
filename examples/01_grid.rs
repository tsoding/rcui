use rcui::*;

fn text_cell(s: &str) -> Box<Text> {
    Box::new(Text {
        text: s.to_string(),
        halign: HAlign::Centre,
        valign: VAlign::Centre,
    })
}

fn main() {
    Rcui::exec(Proxy::wrap(
        |origin, rcui, event| {
            match event {
                Event::KeyStroke(key) => {
                    if *key as u8 as char == 'q' {
                        rcui.quit();
                    }
                }

                _ => {}
            }
            origin.handle_event(rcui, event);
        },
        Column::new(vec![
            Row::wrap(vec![
                text_cell("hello"),
                text_cell("hello"),
                text_cell("hello"),
            ]),
            Row::wrap(vec![
                text_cell("world"),
                text_cell("world"),
                text_cell("world"),
            ]),
            Row::wrap(vec![text_cell("foo"), text_cell("foo"), text_cell("foo")]),
            Row::wrap(vec![text_cell("bar"), text_cell("bar"), text_cell("bar")]),
        ]),
    ));

    println!("Quitting gracefully uwu");
}
