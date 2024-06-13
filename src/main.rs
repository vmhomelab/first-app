use std::string;

use slint::SharedString;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_userInput(move |user_input: SharedString| {
        if let Some(ui) = ui_handle.upgrade() {
            let ergebnis = user_input.clone();
            let ausgabe = format!("Dein Text: {}", ergebnis);
            ui.set_ausgabe(ausgabe.into());
        }
    });

    ui.run()
}
