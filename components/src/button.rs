use gloo_console::log;
use web_sys::HtmlElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub class: AttrValue,
    pub title: AttrValue,
    pub onclick: Callback<AttrValue>,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { class, title, onclick } = props.clone();
    let onclick = Callback::from(move |event: MouseEvent| {
        let val = get_text(event);
        log!(val.as_str());
        onclick.emit(val);
    });
    html!(
        <button type="button" {class} {onclick}>{title}</button>
    )
}

fn get_text(event: MouseEvent) -> AttrValue {
    let event: Event = event.dyn_into().unwrap_throw();
    let event = event.target().unwrap_throw();
    let target: HtmlElement = event.dyn_into().unwrap_throw();
    AttrValue::from(target.inner_text())
}