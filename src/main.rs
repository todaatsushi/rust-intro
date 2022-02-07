mod control_flow;
mod data;
mod functions;
mod greeting;

fn main() {
    greeting::converse();

    data::data_demo();

    functions::about_funcs();

    control_flow::demo_fib();
}
