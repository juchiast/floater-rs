use std;
use yew::prelude::*;

pub type FloatType = u32;
pub const SINGLE_PRECISION: FloatType = 0;
pub const DOUBLE_PRECISION: FloatType = 1;

pub fn make_select<V, F, C, S>(id: &str, title: &str, v: &[(V, &str)], f: F, cur: V) -> Html<C, S>
where
    V: std::fmt::Display + std::str::FromStr + PartialEq,
    <V as std::str::FromStr>::Err: std::fmt::Debug,
    F: 'static + Fn(V) -> <S as Component<C>>::Message,
    S: Component<C> + Renderable<C, S>,
    C: 'static,
{
    let value = move |cd| match cd {
        ChangeData::Select(se) => f(se.raw_value().parse().unwrap()),
        _ => unreachable!(),
    };
    let iter = v.iter().map(|(v, s)| {
        if cur == *v {
            html! {<option value={v}, selected=1,>{s}</option>}
        } else {
            html! {<option value={v},>{s}</option>}
        }
    });
    html! {
        <>
        <label for={id}, >{ title }</label>
        <select id={id}, class="form-control", onchange=|e| value(e), >
            { for iter }
        </select>
        </>
    }
}
