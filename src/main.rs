use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

#[derive(Default)]
struct CalcState {
    prev_value: i32,
    current_value: i32,
    operator: slint::SharedString,
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let state = Rc::new(RefCell::new(CalcState::default()));

    let ui_handle = ui.as_weak();
    ui.global::<Calclogic>().on_button_paressed(move |value| {
        let app = ui_handle.unwrap();

        let mut state: std::cell::RefMut<CalcState> = state.borrow_mut();
        match value.parse::<i32>() {
            Ok(val) => {
                state.current_value *= 10;
                state.current_value += val;
                // app.set_value(state.current_value);
                app.set_value(slint::SharedString::from(
                    state.current_value.to_string().as_str(),
                ));
            }
            Err(_) => {
                if value.as_str() == "=" {
                    let rusult = match state.operator.as_str() {
                        "+" => state.prev_value + state.current_value,
                        "-" => state.prev_value - state.current_value,
                        "*" => state.prev_value * state.current_value,
                        "/" => state.prev_value / state.current_value,
                        _ => state.current_value,
                    };
                    // app.set_value(rusult);
                    app.set_value(slint::SharedString::from(rusult.to_string().as_str()));
                    state.current_value = 0;
                    state.operator = Default::default();
                } else {
                    state.operator = value.clone();
                    state.prev_value = state.current_value;
                    state.current_value = 0;
                }
            }
        };
    });
    ui.run()
}
