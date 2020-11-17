use rcui::*;

fn item_list_controls(item_list: ItemList<String>) -> Box<Proxy<ItemList<String>>> {
    Proxy::wrap(
        |list, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'j' => list.down(),
                'k' => list.up(),
                '\n' => {
                    let item = list.remove();
                    rcui::push_event(Event::Message(item));
                },
                _ => {}
            },
            Event::Message(item) => {
                list.push(item.to_string());
            }
            _ => {}
        },
        item_list)
}

fn main() {
    let n = 10;
    let left_list = ItemList::new((0..n).map(|x| format!("foo-{}", x)).collect());
    let right_list = ItemList::new(Vec::<String>::new());

    rcui::exec(
        Proxy::wrap(
            |row, event| match event {
                Event::KeyStroke(key) => match *key as u8 as char {
                    'q'  => rcui::quit(),
                    '\t' => row.focus_next(),
                    _    => row.handle_event(event),
                }

                Event::Message(_) => {
                    assert!(row.group.widgets.len() == 2);
                    row.group.widgets[1 - row.group.focus].handle_event(event);
                }

                _ => {}
            },
            Row::new(vec![item_list_controls(left_list),
                          item_list_controls(right_list)])));
}
