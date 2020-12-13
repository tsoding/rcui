use rcui::curses::*;
use rcui::*;

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
    Rcui::exec(title(
        "jk to move up and down",
        Proxy::wrap(
            |list, context, event| {
                if let Event::KeyStroke(key) = event {
                    match *key {
                        KEY_NPAGE => list.page_down(),
                        KEY_PPAGE => list.page_up(),
                        key => match key as u8 as char {
                            'q' => context.quit(),
                            'j' => list.down(),
                            'k' => list.up(),
                            _ => {}
                        },
                    }
                }
            },
            ItemList::new((0..100).map(|x| format!("item-{:02}", x)).collect()),
        ),
    ));
    println!("Quitting gracefully uwu");
}
