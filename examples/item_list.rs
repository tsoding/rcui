use rcui::*;

fn main() {
    rcui::exec(Proxy::wrap(
        |list, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => rcui::quit(),
                'j' => list.down(),
                'k' => list.up(),
                _ => {}
            },
        },
        ItemList::new((0..100).map(|x| format!("item-{:02}", x)).collect()),
    ));
    println!("Quiting gracefully uwu");
}
