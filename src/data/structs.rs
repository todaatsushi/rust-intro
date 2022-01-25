fn create_struct() {
    #[derive(Debug)]
    struct Secret {
        x: f64,
        y: i32,
    }

    let new_secret: Secret = Secret { x: 42.1, y: -2 };
    println!(
        "New secret struct with values x: {} and y: {}",
        new_secret.x, new_secret.y
    )
}

pub fn about_structs() {
    println!("\nStructs are like objects and are simple:");
    create_struct();
}
