use wasm_bindgen::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use std::fmt::Write;

#[wasm_bindgen]
pub fn generate_ascii_bubble(message: &str) -> String {
    let mut bubble = String::new();
    let length = message.len() + 4; // padding around message
    
    // top border
    let _ = writeln!(bubble, " {}", "-".repeat(length));
    // Message with padding
    let _ = writeln!(bubble, "< {} >", message);
    // bottom 
    let _ = writeln!(bubble, " {}", "-".repeat(length));
// cat
    let _ = writeln!(bubble, "     \\");
    let _ = writeln!(bubble, "      |\\---/|");
    let _ = writeln!(bubble, "      | ,_, |");
    let _ = writeln!(bubble, "       \\_`_/-..----.");
    let _ = writeln!(bubble, "    ___/ `   ' ,//+ \\  sk");
    let _ = writeln!(bubble, "   (__...'   **\\    |`.**_.';");
    let _ = writeln!(bubble, "     (_,...'(_,.`__)/'.....+\")");
    
    bubble
}

#[wasm_bindgen]
pub fn countdown_then_show_message(message: String) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let countdown_element = document
        .get_element_by_id("countdown")
        .expect("No countdown element found");

    // Mutable countdown value, starts at 3
    let mut countdown_value = 3;

    let closure = Closure::wrap(Box::new(move || {
        if countdown_value > 0 {
            // Update the countdown value in the HTML
            countdown_element.set_inner_html(&countdown_value.to_string());
            countdown_value -= 1;
        } else {
            // After countdown, show the ASCII bubble
            let ascii_bubble = generate_ascii_bubble(&message);
            body.set_inner_html(&format!("<pre>{}</pre>", ascii_bubble));

            // Hide the loading screen and show the message
            document.get_element_by_id("loading").unwrap().set_attribute("style", "display: none;").unwrap();
            document.get_element_by_id("ascii-bubble").unwrap().set_inner_html(&format!("<pre>{}</pre>", ascii_bubble));
        }
    }) as Box<dyn FnMut()>);

    // update the countdown every 1 second (1000 ms)
    let interval_id = window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(), 
            1000  // 1 second interval
        )
        .expect("Should register `setInterval` OK");

    closure.forget();  // Avoid premature closure deallocation
}
