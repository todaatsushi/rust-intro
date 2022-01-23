pub fn lets_do_sums() {
    println!("\nLet's do some sums!\n");
    multiply_nums();
    println!("\nNo more maths.\n");
}

fn multiply_nums() {
    let x: u32 = 42;
    let y = -69; // inferred types
    let xy: i64 = x as i64 * y; // inline casting
    println!("{} * {} = {}", x, y, xy);

    let z: f32 = 3.2;
    println!("{} * {} = {}", z, xy as f32, z * xy as f32);
}
