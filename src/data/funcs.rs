pub fn about_functions() {
    println!("\nFunctions are also data types in themselves.");

    fn f() {
        println!("I am actually func\n")
    }
    let func = f;

    println!("We can assign func to f as a variable.");
    println!("Calling f:");
    f();
    println!("Calling func:");
    func();
}
