fn multiply_and_add_2(x: i32, y: i32) -> i32 {
    let answer = (x * y) + 2;
    println!("Get the answer to {} x {} add 2", x, y);
    return answer;
}

fn add_x_and_y_times_3(x: i32, y: i32) -> i32 {
    println!("Get the answer to {} + {} * 2", x, y);
    (x + y) * 3
}

pub fn about_funcs() {
    println!("Add funcs with args by typing the args and adding to call");
    let answer: i32 = multiply_and_add_2(3, 4);
    println!("The answer: {}", answer);

    let answer2: i32 = add_x_and_y_times_3(10, 100);
    println!("The answer: {}", answer2);
}
