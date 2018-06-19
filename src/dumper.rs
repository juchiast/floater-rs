// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use binary::*;
use select::*;
use stdweb::web::{document, IParentNode};
use yew::prelude::*;

enum Mess {
    Input(String),
    Type(FloatType),
}

struct Model {
    input: String,
    float_type: FloatType,
}

impl Component for Model {
    type Message = Mess;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            input: String::new(),
            float_type: SINGLE_PRECISION,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Mess::Input(s) => self.input = s,
            Mess::Type(t) => self.float_type = t,
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
            { self.view_control() }
            { self.view_result() }
            </>
        }
    }
}

impl Model {
    fn get_dump(&self) -> Result<Dumped, &'static str> {
        use std::str::FromStr;
        if self.input.is_empty() {
            Err("Input is empty!")
        } else {
            match self.float_type {
                SINGLE_PRECISION => match f32::from_str(&self.input) {
                    Err(_) => Err("Input is invalid!"),
                    Ok(f) => Ok(f.dump()),
                },
                DOUBLE_PRECISION => match f64::from_str(&self.input) {
                    Err(_) => Err("Input is invalid!"),
                    Ok(f) => Ok(f.dump()),
                },
                _ => panic!(),
            }
        }
    }

    fn view_control(&self) -> Html<Self> {
        static OPTIONS: &[(FloatType, &str)] = &[
            (SINGLE_PRECISION, "Single-precision"),
            (DOUBLE_PRECISION, "Double-precision"),
        ];
        html! {
            <div class="form-row",>
            <div class="col-sm-auto",>
                <div class="form-group",>
                { make_select("dumper-type", "Floating-point format:", OPTIONS, Mess::Type, self.float_type) }
                </div>
            </div>
            <div class="col-sm",>
                <div class="form-group",>
                <label for="dumper-input",>{ "Input:" }</label>
                <input type="text",
                       class="form-control",
                       id="dumper-input",
                       oninput=|e| Mess::Input(e.value), />
                </div>
            </div>
            </div>
        }
    }
    fn view_result(&self) -> Html<Self> {
        let row = |label, value| {
            html! {
                <tr><td><strong>{label}</strong></td><td style="font-family:monospace",>{value}</td></tr>
            }
        };
        match self.get_dump() {
            Err(s) => html! {
                <div class="alert", class="alert-warning",>{s.to_owned()}</div>
            },
            Ok(d) => html! {
                <table>
                    { row("Value:", d.value) }
                    { row("Sign:", d.sign.to_string()) }
                    { row("Exponent:", format!("{} = {} ({})", d.exp_s, d.exp_biased, d.exp)) }
                    { row("Significand:", d.val) }
                </table>
            },
        }
    }
}

pub fn mount() {
    let app: App<Model> = App::new();
    let element = document().query_selector("#app-dumper").unwrap().unwrap();
    app.mount(element);
}
