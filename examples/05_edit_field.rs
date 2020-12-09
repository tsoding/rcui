use rcui::curses::*;
use rcui::*;

fn main() {
    Rcui::exec(Proxy::wrap(
        |field, rcui, event| {
            if let Event::KeyStroke(key) = event {
                match *key {
                    KEY_LEFT => field.left(),
                    KEY_RIGHT => field.right(),
                    KEY_DC => field.delete_front(),
                    KEY_BACKSPACE => field.delete_back(),
                    // TODO(#50): Replace KEY_F1 and KEY_F2 with Shift+Left and Shift+Right in 05_edit_field
                    KEY_F1 => field.select_left(),
                    KEY_F2 => field.select_right(),
                    KEY_F3 => field.put_selection_to_clipboard(rcui),
                    KEY_F4 => field.paste_from_clipboard(rcui),
                    _ => {
                        if *key as u8 as char == '\n' {
                            rcui.quit()
                        } else {
                            field.handle_event(rcui, event)
                        }
                    }
                }
            }
        },
        EditField::new(),
    ))
}
