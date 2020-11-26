[![Build Status](https://github.com/tsoding/rcui/workflows/CI/badge.svg)](https://github.com/tsoding/rcui/actions)

# rcui

Simple TUI framework in Rust.

## Example

Item List with 100 elements and vim-like navigation

```rust
use rcui::*;

fn main() {
    Rcui::exec(Proxy::wrap(
        |list, context, event| match event {
            Event::KeyStroke(key) => match *key as u8 as char {
                'q' => context.quit(),
                'j' => list.down(),
                'k' => list.up(),
                _ => {}
            }

            _ => {}
        },
        ItemList::new((0..100).map(|x| format!("item-{:02}", x)).collect()),
    ));
    println!("Quitting gracefully uwu");
}
```

## Quick Start

```console
$ cargo run --example 01_grid
$ cargo run --example 02_item_list
```
