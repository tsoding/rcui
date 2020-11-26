use rcui::*;

fn main() {
    Rcui::exec(Proxy::wrap(
        |list, context, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => context.quit(),
                'j' => list.down(),
                'k' => list.up(),
                _ => {}
            },

            _ => {}
        },
        ItemList::new((0..100).map(|x| format!("item-{:02}", x)).collect()),
    ));
    println!("Quitting gracefully uwu");
}
