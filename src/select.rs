use yew::prelude::*;

pub type FloatType = u32;
pub const SINGLE_PRECISION: FloatType = 0;
pub const DOUBLE_PRECISION: FloatType = 1;

pub fn make_select<V, F, S>(id: &str, title: &str, v: &[(V, &str)], f: F, cur: V) -> Html<S>
where
    V: std::fmt::Display + std::str::FromStr + PartialEq,
    <V as std::str::FromStr>::Err: std::fmt::Debug,
    F: 'static + Fn(V) -> <S as Component>::Message,
    S: Component + Renderable<S>,
{
    let value = move |cd| match cd {
        ChangeData::Select(se) => f(se.raw_value().parse().unwrap()),
        _ => unreachable!(),
    };
    let iter = v
        .iter()
        .map(|(v, s)| html! {<option value={v}, selected=cur == *v,>{s}</option>});
    html! {
        <>
        <label for={id}, >{ title }</label>
        <select id={id}, class="form-control", onchange=|e| value(e), >
            { for iter }
        </select>
        </>
    }
}
