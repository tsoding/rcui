# rcui

Simple TUI framework in Rust inspired by Qt.

## Example

```rust
use rcui::*;

fn main() {
    rcui::exec(Proxy::wrap(
        |root, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => rcui::quit(),
                's' => root.down(),
                'w' => root.up(),
                _ => {}
            },
        },
        ItemList::new(vec![
            "foo", "bar", "baz"
        ]),
    ));
    println!("Quiting gracefully uwu");
}
```

## Quick Start

```console
$ cargo run --example grid
$ cargo run --example item_list
```
