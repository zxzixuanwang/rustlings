// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();
    get_char(&data);
    
    string_uppercase(&data);

}

// Should not take ownership
fn get_char<'a>(data:  &'a String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase<'a>(data: &'a String) {
    let data = &data.to_uppercase();

    println!("{}", data);
}
