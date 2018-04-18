extern crate stdweb;
#[macro_use]
extern crate yew;

mod binary;
mod builder;
mod dumper;

fn main() {
    yew::initialize();
    dumper::mount();
    builder::mount();
    yew::run_loop();
}
