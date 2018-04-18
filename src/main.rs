#![recursion_limit = "128"]

extern crate stdweb;
#[macro_use]
extern crate yew;

mod binary;
mod builder;
mod dumper;

fn main() {
    yew::initialize();
    dumper::Model::mount();
    yew::run_loop();
}
