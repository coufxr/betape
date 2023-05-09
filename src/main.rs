use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

#[derive(Default)]
struct CalcState {
    prev_value: slint::SharedString,
    current_value: slint::SharedString,
    operator: slint::SharedString,
}

// fn join_str(mut state: std::cell::RefMut<CalcState>, val: i32) -> std::cell::RefMut<'_, CalcState> {
//     let  Ok(mut a) = state.prev_value.parse::<i32>() else { todo!() };
    
//     a *= 10;
//     a += val;
//     state.current_value=slint::SharedString::from(a.to_string().as_str());
//     state
// }

// fn calc_str(
//     mut state: std::cell::RefMut<'_, CalcState>,
// ) -> (std::cell::RefMut<'_, CalcState>, i32) {
//     let Ok(a) = state.prev_value.parse::<i32>() else { todo!() };
//     let Ok(b) = state.current_value.parse::<i32>() else { todo!() };
//     let rusult = match state.operator.as_str() {
//         "+" => a+b,
//         "-" => a-b,
//         "*" => a*b,
//         "/" => a/b,
//         &_ => todo!(),
//     };
//     state.current_value = Default::default();
//     state.operator = Default::default();
//     (state, rusult)
// }

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let state = Rc::new(RefCell::new(CalcState::default()));

    let ui_handle = ui.as_weak();
    ui.global::<Calclogic>().on_button_paressed(move |value| {
        let app = ui_handle.unwrap();

        let mut state: std::cell::RefMut<CalcState> = state.borrow_mut();
        match value.parse::<i32>() {
            Ok(val) => {
                let  Ok(mut a) = state.prev_value.parse::<i32>() else { todo!() };
                a *= 10;
                a += val;
                state.current_value=slint::SharedString::from(a.to_string().as_str());
            }
            Err(_) => {
                let Ok(a) = state.prev_value.parse::<i32>() else { todo!() };
                let Ok(b) = state.current_value.parse::<i32>() else { todo!() };
                let rusult = match state.operator.as_str() {
                    "+" => a+b,
                    "-" => a-b,
                    "*" => a*b,
                    "/" => a/b,
                    &_ => todo!(),
                };
                state.current_value = Default::default();
                state.operator = Default::default();
                app.set_value(slint::SharedString::from(rusult.to_string().as_str()));
            },
        };

        let value = &value;
        app.set_value(app.get_value() + value);
    });
    ui.run()
}
