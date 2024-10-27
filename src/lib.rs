use wasm_bindgen::prelude::*;
use web_sys::window;
use wasm_bindgen::JsCast;
use std::fmt::Write;

#[wasm_bindgen]
#[derive(Clone)]
pub enum Animal {
    Cat,
    Cow,
    Dog,
    Monkey
}

#[wasm_bindgen]
pub fn generate_ascii_bubble(message: &str, selected_animal: &str) -> String {  // added selected_animal parameter
    let mut bubble = String::new();
    let length = message.len() + 4; // padding around message

    // top border
    let _ = writeln!(bubble, " {}", "-".repeat(length));
    // Message with padding
    let _ = writeln!(bubble, "< {} >", message);
    // bottom
    let _ = writeln!(bubble, " {}", "-".repeat(length));
    
    // Now we can match on selected_animal
    let ascii_art = match selected_animal {
        "cat" => String::from(
            " |\\\\---/|\n\
             | ,*, |\n\
             \\\\*`_/-..----.\n\
             *__/ ` ' ,//+ \\\\ sk\n\
             (*_...' **\\\\ |`.**_.'\n\
             (*,...'(_,.`__)/'.....+\")"
        ),
        "cow" => String::from(
            "        \\\\   ^__^\n\
                     \\\\  (oo)\\\\_______\n\
                        (__)\\\\       )\\\\/\\\\\n\
                            ||----w |\n\
                            ||     ||"
        ),
        "dog" => String::from(
            "    ''',\n\
        o_)O \\)____)\n\
         \\_        )\n\
           '',,,,,,\n\
             ||  ||\n\
            \"--'\"--'\"\n\
                )"
        ),
        "monkey" => String::from(
            "  /\\\\___/\\\\\n\
             (  o o  )\n\
             (  =^=  )\n\
              (m___m)"
        ),
        _ => String::from(  // Default to cat if unknown animal type
            " |\\\\---/|\n\
             | ,*, |\n\
             \\\\*`_/-..----.\n\
             *__/ ` ' ,//+ \\\\ sk\n\
             (*_...' **\\\\ |`.**_.'\n\
             (*,...'(_,.`__)/'.....+\")"
        )
    };
    
    bubble.push_str(&ascii_art);
    bubble
}

#[wasm_bindgen]
pub fn countdown_then_show_message(message: String, animal_type: String) {  // Added animal_type parameter
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let countdown_element = document
        .get_element_by_id("countdown")
        .expect("No countdown element found");

    let mut countdown_value = 3;
    let closure = Closure::wrap(Box::new(move || {
        if countdown_value > 0 {
            countdown_element.set_inner_html(&countdown_value.to_string());
            countdown_value -= 1;
        } else {
            let ascii_bubble = generate_ascii_bubble(&message, &animal_type);  
            body.set_inner_html(&format!("<pre>{}</pre>", ascii_bubble));
            document.get_element_by_id("loading").unwrap().set_attribute("style", "display: none").unwrap();
            document.get_element_by_id("ascii-bubble").unwrap().set_inner_html(&format!("<pre>{}</pre>", ascii_bubble));
        }
    }) as Box<dyn FnMut()>);

    let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        700
    );
    closure.forget();
}