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
            Cell::Many(Row::wrap(vec![
                Cell::Many(text_cell("hello"), 1),
                Cell::One(text_cell("hello")),
                Cell::One(text_cell("hello")),
            ]), 3),
            Cell::Many(Row::wrap(vec![
                Cell::One(text_cell("world")),
                Cell::One(text_cell("world")),
                Cell::One(text_cell("world")),
            ]), 2),
            Cell::One(Row::wrap(vec![
                Cell::One(text_cell("foo")),
                Cell::One(text_cell("foo")),
                Cell::One(text_cell("foo"))
            ])),
            Cell::One(Row::wrap(vec![
                Cell::One(text_cell("bar")),
                Cell::One(text_cell("bar")),
                Cell::One(text_cell("bar"))
            ])),
        ]),
    ));

    println!("Quitting gracefully uwu");
}
