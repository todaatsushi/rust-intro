fn say_goodbye() {
    println!("Goodbye!");
}

fn say_hello() {
    println!("Hello, world!");
}

fn get_time() {
    println!("The current time is {}", chrono::Local::now());
}

pub fn converse() {
    say_hello();
    get_time();
    say_goodbye();
}
