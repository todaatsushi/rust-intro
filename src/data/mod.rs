mod arrays;
mod booleans;
mod enums;
mod funcs;
mod misc;
mod numbers;
mod strings;
mod structs;
mod tuples;

fn show_simple_data() {
    numbers::lets_do_sums();
    booleans::show_bools();
    strings::show_me_them_strings();
    funcs::about_functions();
}

fn complex_data_info() {
    tuples::about_tuples();
    arrays::about_arrays();
    structs::about_structs();
    enums::show_me_enums();

    misc::about_misc_data_types();
}

pub fn data_demo() {
    show_simple_data();
    complex_data_info();
}
