// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::binary::*;
use crate::select::*;
use stdweb::web::{document, IParentNode};
use yew::prelude::*;

enum Msg {
    Input(String),
    Type(FloatType),
}

struct Model {
    input: String,
    float_type: FloatType,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            input: String::new(),
            float_type: SINGLE_PRECISION,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(s) => self.input = s,
            Msg::Type(t) => self.float_type = t,
        }
        true
    }

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
    fn view_control(&self) -> Html<Self> {
        static OPTIONS: &[(FloatType, &str)] = &[
            (SINGLE_PRECISION, "Single-precision"),
            (DOUBLE_PRECISION, "Double-precision"),
        ];
        html! {
            <div class="form-row">
            <div class="col-sm-auto">
                <div class="form-group">
                { make_select("builder-type", "Floating-point format:", OPTIONS, Msg::Type, self.float_type) }
                </div>
            </div>
            <div class="col-sm">
                <div class="form-group">
                <label for="builder-input">{ "Input:" }</label>
                <input type="text"
                       class="form-control"
                       id="builder-input"
                       oninput=|e| Msg::Input(e.value) />
                </div>
            </div>
            </div>
        }
    }
    fn view_result(&self) -> Html<Self> {
        if self.input.is_empty() {
            return html! {
                <div class="alert alert-warning">{"Input is empty!"}</div>
            };
        }

        html! {
            <p>
            { "Result: " }
            {
                match self.float_type {
                    SINGLE_PRECISION => match f32::build(&self.input) {
                        Err(s) => s.to_owned(),
                        Ok(f) => format!("{:e}", f),
                    }
                    DOUBLE_PRECISION => match f64::build(&self.input) {
                        Err(s) => s.to_owned(),
                        Ok(f) => format!("{:e}", f),
                    }
                    _ => panic!(),
                }
            }
            </p>
        }
    }
}

pub fn mount() {
    let app: App<Model> = App::new();
    let element = document().query_selector("#app-builder").unwrap().unwrap();
    app.mount(element);
}
