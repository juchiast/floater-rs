# floater-rs
An experimental project on writing pure-rust client web app.

This project uses cutting-edge technologies to build a floating-point tool, capable of viewing
binary presentation and constructing floating-point number from binary.

In detail, it uses [Yew](https://github.com/DenisKolodin/yew), a [Rust](https://www.rust-lang.org/en-US/)
framework for writing client web app. Rust code is compiled to [WebAssembly](http://webassembly.org)
using [cargo-web](https://github.com/koute/cargo-web), then served in the browser.

## Usage

You can use the app here: https://vietcodes.github.io/tools/floater

There is also a pre-built image in `./dist`, all you have to do is serve the `./dist` directory
using `http-server` or similar tools.

**Note:** These files must be served by a web server, simply open `index.html` doesn't work.

## Build

Prepare the build environment:
- Install `rustup`, as in https://rustup.rs
- Install Rust nightly: `rustup install nightly`
- Switch to Rust nightly: `rustup default nightly`
- Install the `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Install `cargo-web`: `cargo install cargo-web`

To build, run `make`, which does the following things:
- `cargo web build`
- Copy output files to `./dist`

## License

Licensed under either of
- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are welcomed, if you have any questions, new ideas, bug reports, etc., please open issue or pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
