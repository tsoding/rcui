use rcui::*;

fn item_list_controls<T: ToString + Clone>(item_list: ItemList<T>) -> Box<Proxy<ItemList<T>>> {
    Proxy::wrap(
        |list, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'j' => list.down(),
                'k' => list.up(),
                _ => {}
            }

            _ => {}
        },
        item_list)
}

fn main() {
    let n = 100;
    let left_list = ItemList::new((0..n).map(|x| format!("foo-{}", x)).collect());
    let right_list = ItemList::new((0..n).map(|x| format!("bar-{}", x)).collect());
    rcui::exec(
        Proxy::wrap(
            |hbox, event| match event {
                Event::KeyStroke(key) => match *key as u8 as char {
                    'q' => rcui::quit(),
                    '\t' => hbox.focus_next(),
                    _ => hbox.handle_event(event),
                }

                _ => {}
            },
            Row::new(
                vec![item_list_controls(left_list),
                     item_list_controls(right_list)])));
}
