use rcui::*;

fn main() {
    Rcui::exec(
        Proxy::wrap(
            |field, rcui, event| {
                if let Event::KeyStroke(key) = event {
                    if *key as u8 as char == '\n' {
                        rcui.quit()
                    }
                }
                field.handle_event(rcui, event)
            },
            EditField::new()
        )
    )
}
