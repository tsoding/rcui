use rcui::{vbox, hbox, text, ItemList};

fn main() {
    rcui::exec(
        vbox(vec![
            hbox(vec![
                Box::new(ItemList {
                    items: vec!["item1", "item2", "item3"]
                }),
                text("hello"),
                text("hello"),
            ]),
            hbox(vec![ text("world"), text("world"), text("world") ]),
            hbox(vec![ text("foo"), text("foo"), text("foo") ]),
            hbox(vec![ text("bar"), text("bar"), text("bar") ]),
        ])
    );
}
