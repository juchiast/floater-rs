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
    fn get_dump(&self) -> Result<Dumped, &'static str> {
        use std::str::FromStr;
        if self.input.is_empty() {
            Err("Input is empty!")
        } else {
            match self.float_type {
                FloatType::Single => match f32::from_str(&self.input) {
                    Err(_) => Err("Input is invalid!"),
                    Ok(f) => Ok(f.dump()),
                },
                FloatType::Double => match f64::from_str(&self.input) {
                    Err(_) => Err("Input is invalid!"),
                    Ok(f) => Ok(f.dump()),
                },
            }
        }
    }

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
                <label for="dumper-type",>{ "Floating-point format:" }</label>
                <select class="form-control", id="dumper-type",>
                { option(FloatType::Single, "Single-precision") }
                { option(FloatType::Double, "Double-precision") }
                </select>
                </div>
            </div>
            <div class="col-sm",>
                <div class="form-group",>
                <label for="dumper-input",>{ "Input:" }</label>
                <input type="text",
                       class="form-control",
                       id="dumper-input",
                       oninput=|e: InputData| Mess::Input(e.value), />
                </div>
            </div>
            </div>
        }
    }
    fn view_result(&self) -> Html<Context, Self> {
        let row = |label, value| html! {
            <tr><td><strong>{label}</strong></td><td style="font-family:monospace",>{value}</td></tr>
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
    let app: App<_, Model> = App::new(());
    let element = document().query_selector("#app-dumper").unwrap().unwrap();
    app.mount(element);
}
