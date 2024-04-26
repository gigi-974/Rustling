// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);  // ajout de & pour passer une référence

    string_uppercase(data);  // suppression de & pour prendre la propriété
}

// Should not take ownership
fn get_char(data: &String) -> char { // String au lieu de &String
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {  // &String au lieu de String
    data = data.to_uppercase();

    println!("{}", data);
}
