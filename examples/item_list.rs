use rcui::*;

fn main() {
    rcui::exec(
        HardProxy::wrap(
            |root, event| {
                match event {
                    Event::KeyStroke(key) => {
                        match *key as u8 as char {
                            'q' => rcui::quit(),
                            's' => root.down(),
                            'w' => root.up(),
                            _ => {}
                        }
                    }
                }
            },
            ItemList::new(vec!["foo", "bar", "baz"])
        )
    );
    println!("Quiting gracefully uwu");
}
