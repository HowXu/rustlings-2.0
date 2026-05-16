#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
// cause you are data = xxx
// here data is same as the return of to_uppercase(&self) -> String
fn string_uppercase(data: &String) {
    // data = data.to_uppercase();
    // change like this will make things different
    // here we needn't mut data cause we make a new one
    let data_ref = data.to_uppercase();

    println!("{data_ref}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(&data);
}
