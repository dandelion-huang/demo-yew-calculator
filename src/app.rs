use yew::prelude::*;

// Define the operations that can be performed. Traits: Clone, PartialEq
#[derive(Clone, PartialEq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[function_component]
pub fn App() -> Html {
    let result = use_state(|| 0f64);
    let current_input = use_state(String::new);
    let operation = use_state(|| None::<Operation>);
    let waiting_for_operand = use_state(|| false);

    let update_input = {
        let current_input = current_input.clone();
        let waiting_for_operand = waiting_for_operand.clone();
        Callback::from(move |digit: char| {
            if *waiting_for_operand {
                current_input.set(digit.to_string());
                waiting_for_operand.set(false);
            } else {
                current_input.set(format!("{}{}", *current_input, digit));
            }
        })
    };

    let clear = {
        let result = result.clone();
        let current_input = current_input.clone();
        let operation = operation.clone();
        let waiting_for_operand = waiting_for_operand.clone();
        Callback::from(move |_| {
            result.set(0f64);
            current_input.set(String::new());
            operation.set(None);
            waiting_for_operand.set(false);
        })
    };

    let perform_operation = {
        let result = result.clone();
        let current_input = current_input.clone();
        let operation = operation.clone();
        move || {
            if let Some(op) = &*operation {
                let current_value = current_input.parse::<f64>().unwrap_or(0.0);
                let new_result = match op {
                    Operation::Add => *result + current_value,
                    Operation::Subtract => *result - current_value,
                    Operation::Multiply => *result * current_value,
                    Operation::Divide => if current_value != 0.0 { *result / current_value } else { *result },
                };
                result.set(new_result);
                current_input.set(new_result.to_string());
            } else {
                let current_value = current_input.parse::<f64>().unwrap_or(0.0);
                result.set(current_value);
            }
        }
    };

    let set_operation = {
        let operation = operation.clone();
        let waiting_for_operand = waiting_for_operand.clone();
        let perform_operation = perform_operation.clone();
        Callback::from(move |op: Operation| {
            perform_operation();
            operation.set(Some(op));
            waiting_for_operand.set(true);
        })
    };

    let calculate = {
        let operation = operation.clone();
        let waiting_for_operand = waiting_for_operand.clone();
        let perform_operation = perform_operation.clone();
        Callback::from(move |_| {
            perform_operation();
            operation.set(None);
            waiting_for_operand.set(true);
        })
    };

    html! {
        <div class="content">
        <div class="calculator">
            <div class="result">{format!("{:.2}", if current_input.is_empty() { *result } else { current_input.parse().unwrap_or(0.0) })}</div>
            <div class="buttons">
                <button class="btn" onclick={update_input.reform(|_| '7')}>{"7"}</button>
                <button class="btn" onclick={update_input.reform(|_| '8')}>{"8"}</button>
                <button class="btn" onclick={update_input.reform(|_| '9')}>{"9"}</button>
                <button class="btn" onclick={set_operation.reform(|_| Operation::Divide)}>{"/
"}</button>
                <button class="btn" onclick={update_input.reform(|_| '4')}>{"4"}</button>
                <button class="btn" onclick={update_input.reform(|_| '5')}>{"5"}</button>
                <button class="btn" onclick={update_input.reform(|_| '6')}>{"6"}</button>
                <button class="btn" onclick={set_operation.reform(|_| Operation::Multiply)}>{"*"}</button>
                <button class="btn" onclick={update_input.reform(|_| '1')}>{"1"}</button>
                <button class="btn" onclick={update_input.reform(|_| '2')}>{"2"}</button>
                <button class="btn" onclick={update_input.reform(|_| '3')}>{"3"}</button>
                <button class="btn" onclick={set_operation.reform(|_| Operation::Subtract)}>{"-"}</button>
                <button class="btn" onclick={update_input.reform(|_| '0')}>{"0"}</button>
                <button class="btn" onclick={update_input.reform(|_| '.')}>{"."}</button>
                <button class="btn" onclick={calculate}>{"="}</button>
                <button class="btn" onclick={set_operation.reform(|_| Operation::Add)}>{"+"}</button>
                <button class="btn btn-clear" onclick={clear}>{"C"}</button>
            </div>
        </div>
    </div>
    }
}