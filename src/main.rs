// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate stdweb;
#[macro_use]
extern crate yew;

mod binary;
mod builder;
mod dumper;
mod select;

fn main() {
    yew::initialize();
    dumper::mount();
    builder::mount();
    yew::run_loop();
}
