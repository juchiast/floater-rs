// Copyright 2018 Đỗ Hoàng Anh Duy.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use binary::*;
use stdweb::web::{document, IParentNode};
use yew::prelude::*;

type Context = ();

#[derive(PartialEq, Clone, Copy)]
enum FloatType {
    Single,
    Double,
}

enum Mess {
    Input(String),
    Type(FloatType),
}

struct Model {
    input: String,
    float_type: FloatType,
}

impl Component<Context> for Model {
    type Msg = Mess;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            input: String::new(),
            float_type: FloatType::Single,
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Mess::Input(s) => self.input = s,
            Mess::Type(t) => self.float_type = t,
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <>
            { self.view_control() }
            { self.view_result() }
            </>
        }
    }
}

impl Model {
    fn view_control(&self) -> Html<Context, Self> {
        let option = |t, s| {
            if t == self.float_type {
                html! { <option onclick=move |_| Mess::Type(t), selected=1,>{ s }</option> }
            } else {
                html! { <option onclick=move |_| Mess::Type(t),>{ s }</option> }
            }
        };
        html! {
            <div class="form-row",>
            <div class="col-sm-auto",>
                <div class="form-group",>
                <label for="builder-type",>{ "Floating-point format:" }</label>
                <select class="form-control", id="builder-type",>
                { option(FloatType::Single, "Single-precision") }
                { option(FloatType::Double, "Double-precision") }
                </select>
                </div>
            </div>
            <div class="col-sm",>
                <div class="form-group",>
                <label for="builder-input",>{ "Input:" }</label>
                <input type="text",
                       class="form-control",
                       id="builder-input",
                       oninput=|e: InputData| Mess::Input(e.value), />
                </div>
            </div>
            </div>
        }
    }
    fn view_result(&self) -> Html<Context, Self> {
        if self.input.is_empty() {
            return html! {
                <div class="alert", class="alert-warning",>{"Input is empty!"}</div>
            };
        }

        html! {
            <p>
            { "Result: " }
            {
                match self.float_type {
                    FloatType::Single => match f32::build(&self.input) {
                        Err(s) => s.to_owned(),
                        Ok(f) => format!("{:e}", f),
                    }
                    FloatType::Double => match f64::build(&self.input) {
                        Err(s) => s.to_owned(),
                        Ok(f) => format!("{:e}", f),
                    }
                }
            }
            </p>
        }
    }
}

pub fn mount() {
    let app: App<_, Model> = App::new(());
    let element = document().query_selector("#app-builder").unwrap().unwrap();
    app.mount(element);
}
