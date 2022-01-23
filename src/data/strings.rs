fn my_name_is() {
    let name = "Atsushi";
    println!("My name is {}.", name);
}

fn best_char() {
    let best_char: char = 'x'; // number of quote marks
    println!("I like {}.", best_char);
    println!("In bits: {}.", best_char as u32);
    println!("As hex: U+{:X}", best_char as u32);
}

pub fn show_me_them_strings() {
    println!("\nStrings:\n");
    my_name_is();
    best_char();
}
