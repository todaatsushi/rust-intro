pub fn show_bools() {
    println!("\nNow bools.\n");
    let is_true: bool = -1 < 0;
    println!("{} < {}: {}", -1, 0, is_true);

    let is_false = -1 > 0;
    println!("{} > {}: {}", -1, 0, is_false);
}
