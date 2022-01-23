mod data;
mod greeting;

fn main() {
    greeting::converse();

    tuples::about_tuples();
    data::show_simple_data();
}

mod tuples {
    pub fn about_tuples() {
        println!("\nTuples now...\n");
        let tuple_example: (u32, u32) = (1, 2); // have to be the same type
        println!("Tuples look like {:?}.", tuple_example); // :? to print
    }
}
