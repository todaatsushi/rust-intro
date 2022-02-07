mod control_flow;
mod data;
mod functions;
mod greeting;
mod mob;

fn main() {
    greeting::converse();

    data::data_demo();

    functions::about_funcs();

    control_flow::demo_fib();

    mob::talk_about_mob();
}
