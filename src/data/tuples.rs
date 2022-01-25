pub fn about_tuples() {
    println!("\nTuples now...\n");
    let tuple_example: (f32, u32) = (1.1, 2); // can be any type
    println!("Tuples look like {:?}.", tuple_example); // :? to print

    println!("To fetch an element: {}", tuple_example.0)
}
