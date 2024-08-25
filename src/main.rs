use slint::SharedString;
use base64::prelude::*;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let mut clipboard = ClipboardContext::new().unwrap();

    ui.on_encrypt({
        let ui_handle = ui_handle.clone();

        move |string: SharedString| {
            if let Some(ui) = ui_handle.upgrade() {
                // Encode the string
                let password_bytes = string.as_bytes();
                let encoded = base64::engine::general_purpose::STANDARD.encode(password_bytes);
                let result = SharedString::from(encoded);

                ui.set_results(result.clone());

                if let Err(e) = clipboard.set_contents(result.to_string()) {
                    eprintln!("Failed to copy to clipboard: {}", e);
                }
            }
        }
    });

    ui.run()
}