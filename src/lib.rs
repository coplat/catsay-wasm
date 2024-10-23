use wasm_bindgen::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use std::fmt::Write;

#[wasm_bindgen]
pub fn generate_ascii_bubble(message: &str) -> String {
    let mut bubble = String::new();
    let length = message.len() + 4; // Adds padding around message
    
    // Top border
    let _ = writeln!(bubble, " {}", "-".repeat(length));
    // Message with padding
    let _ = writeln!(bubble, "< {} >", message);
    // Bottom border
    let _ = writeln!(bubble, " {}", "-".repeat(length));
    
    bubble
}

#[wasm_bindgen]
pub fn countdown_then_show_message(message: String) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    //closure for the countdown
    let closure = Closure::wrap(Box::new(move || {
        // After countdown, show the ASCII bubble
        let ascii_bubble = generate_ascii_bubble(&message);
        body.set_inner_html(&format!("<pre>{}</pre>", ascii_bubble));
    }) as Box<dyn FnMut()>);

    // `unchecked_ref()` to convert closure to a functio reference expected by `setTimeout`
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(), // Cast closure to `&Function`
            3000, // 3 seconds timeout
        )
        .expect("Should register `setTimeout` OK");

    closure.forget(); // prevent premature closure deallocation 
}