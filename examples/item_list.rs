use rcui::*;

fn main() {
    rcui::exec(Proxy::wrap(
        |origin, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => rcui::quit(),
                's' => origin.down(),
                'w' => origin.up(),
                _ => {}
            },
        },
        ItemList::new(vec![
            "foo", "bar", "baz", "test", "hello", "world", "dfsdjf", "sdfjksdf",
        ]),
    ));
    println!("Quiting gracefully uwu");
}
