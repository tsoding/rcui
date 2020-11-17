use rcui::*;

fn main() {
    let n = 20;
    rcui::exec(
        Proxy::wrap(
            |_hbox, event| match event {
                Event::KeyStroke(key) => match *key as u8 as char {
                    'q' => rcui::quit(),
                    '\t' => todo!(),
                    _ => {}
                },
            },
            HBox::new(
                vec![ItemList::wrap((0..n).map(|x| format!("foo-{}", x)).collect()),
                     ItemList::wrap((0..n).map(|x| format!("bar-{}", x)).collect())])));
}
