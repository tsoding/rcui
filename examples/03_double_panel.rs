use rcui::*;

fn item_list_controls<T: ToString + Clone>(item_list: ItemList<T>) -> Box<Proxy<ItemList<T>>> {
    Proxy::wrap(
        |list, _, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'j' => list.down(),
                'k' => list.up(),
                _ => {}
            },

            _ => {}
        },
        item_list,
    )
}

fn title(title: &str, widget: Box<dyn Widget>) -> Box<dyn Widget> {
    let mut title = Column::wrap(vec![
        Cell::Fixed(
            3.0,
            Box::new(Text {
                text: title.to_string(),
                halign: HAlign::Centre,
                valign: VAlign::Centre,
            }),
        ),
        Cell::One(widget),
    ]);
    title.group.focus = 1;
    title
}

fn main() {
    let n = 100;
    let left_list = ItemList::new((0..n).map(|x| format!("foo-{}", x)).collect());
    let right_list = ItemList::new((0..n).map(|x| format!("bar-{}", x)).collect());
    Rcui::exec(title(
        "jk to move up and down, TAB to switch the focus",
        Proxy::wrap(
            |hbox, context, event| match event {
                Event::KeyStroke(key) => match *key as u8 as char {
                    'q' => context.quit(),
                    '\t' => hbox.focus_next(),
                    _ => hbox.handle_event(context, event),
                },

                _ => {}
            },
            Row::new(vec![
                Cell::One(item_list_controls(left_list)),
                Cell::One(item_list_controls(right_list)),
            ]),
        ),
    ));
}
