// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    let lastChar = get_char(&data);
    println!("{}", lastChar);

    let upperCaseData = string_uppercase(&data);
    println!("{}", upperCaseData);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) -> String {
    let data = data.to_uppercase();
    data
}
