use rcui::*;

struct AddItem {
    label: String,
}

fn item_list_controls(item_list: ItemList<String>) -> Box<Proxy<ItemList<String>>> {
    Proxy::wrap(
        |list, context, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'j' => list.down(),
                'k' => list.up(),
                '\n' => {
                    let item = list.remove();
                    context.push_event(Event::Custom(Box::new(AddItem { label: item })));
                }
                _ => {}
            },
            Event::Custom(event) => {
                if let Some(add_item) = event.downcast_ref::<AddItem>() {
                    list.push(add_item.label.clone());
                }
            }
            _ => {}
        },
        item_list,
    )
}

fn main() {
    let n = 10;
    let left_list = ItemList::new((0..n).map(|x| format!("foo-{}", x)).collect());
    let right_list = ItemList::new(Vec::<String>::new());

    Rcui::exec(Proxy::wrap(
        |row, context, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => context.quit(),
                '\t' => row.focus_next(),
                _ => row.handle_event(context, event),
            },

            Event::Custom(_) => {
                assert!(row.group.cells.len() == 2);
                row.group.cells[1 - row.group.focus]
                    .get_widget_mut()
                    .handle_event(context, event);
            }

            _ => {}
        },
        Row::new(vec![
            Cell::One(item_list_controls(left_list)),
            Cell::One(item_list_controls(right_list)),
        ]),
    ));
}
