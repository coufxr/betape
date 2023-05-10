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
    ui.global::<Calclogic>()
        .on_button_paressed(move |value, typ| {
            let app = ui_handle.unwrap();

            let mut state: std::cell::RefMut<CalcState> = state.borrow_mut();

            match typ {
                1 => {
                    let val = value.parse::<i32>().unwrap();
                    // state.current_value *= 10;

                    let new_val = format!("{}{}", app.get_value(), value);
                    app.set_value(slint::SharedString::from(new_val));
                    state.current_value = val;
                }
                2 => {
                    let val = format!("{}{}", app.get_value(), value);
                    app.set_value(slint::SharedString::from(val));
                }
                3 => {
                    if value == "AC" {
                        state.current_value = Default::default();
                        state.prev_value = Default::default();
                        state.operator = Default::default();
                        app.set_value(slint::SharedString::new());
                    } else {
                        state.current_value = Default::default();
                        let text_value = app.get_value();
                        let a: &str = &text_value[0..text_value.len() - 1];
                        app.set_value(slint::SharedString::from(a));
                    }
                }
                4 => {
                    let prev: i32 = state.prev_value.to_string().parse::<i32>().unwrap();
                    let new: i32 = -(prev);
                    // 字符串的最后一位是最后输入的数字
                    match app.get_value().to_string().pop().unwrap().to_string().parse::<i32>() {
                        Ok(end_str) =>{
                            if end_str == prev {
                                state.prev_value = new;
                                app.set_value(slint::SharedString::from(format!("{value}{new}")));
                            }
                        },
                        Err(_)=>todo!()
                    }
                }
                5 => {
                    // todo: 需要补齐计算逻辑
                    let rusult = 15;
                    app.set_value(slint::SharedString::from("Null".to_string().as_str()));
                    state.current_value = rusult;
                    state.operator = Default::default();
                }
                _ => todo!(),
            }
        });
    ui.run()
}
