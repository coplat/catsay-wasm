use clap::Parser;
use wasm_bindgen::prelude::*;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

#[wasm_bindgen]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    message: String,
}
#[wasm_bindgen]
fn frame(message: String, decorator: char) -> std::string::String { 
    // Count Length of Message 
    let length = message.len() + 2;
    // Return a decorator repeated at message length
    let mut border = String::from("");
    
    for _n in 1..length {
        border.push(decorator);
    }
    return border 
} 
#[wasm_bindgen] 
fn main() {
    let args = Args::parse();
    println!(" {} ", frame(args.message.clone(), '_'));
    println!("< {} >", args.message);
    println!(" {} ",frame(args.message, '-'));
    println!("   |\\---/|
   | ,_, |
    \\_`_/-..----.
 ___/ `   ' ,//+ \\  sk
(__...'   __\\    |`.___.';
  (_,...'(_,.`__)/'.....+");
}
