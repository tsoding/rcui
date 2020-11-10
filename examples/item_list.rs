use rcui::*;

fn main() {
    rcui::exec(
        Proxy::wrap(
            |event| {
                match event {
                    Event::KeyStroke(key) => {
                        if *key as u8 as char == 'q' {
                            rcui::quit()
                        }
                    }
                }
            },
            Box::new(
                ItemList::new(vec!["foo", "bar", "baz"])
            )
        )
    );
    println!("Quiting gracefully uwu");
}
