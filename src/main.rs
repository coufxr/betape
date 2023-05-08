slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.global::<Calclogic>().on_button_paressed(move |value| {
        let app = ui_handle.unwrap();
        let value = &value;
        app.set_value(app.get_value()+value);
    });
    ui.run()
}
