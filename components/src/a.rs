use web_sys::HtmlElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, PartialEq, Properties)]
pub struct AProps {
    pub class: AttrValue,
    pub title: AttrValue,
    pub onclick: Callback<AttrValue>,
}

#[function_component]
pub fn A(props: &AProps) -> Html {
    let AProps{ class, title,  onclick } = props.clone();
    let onclick = Callback::from(move |event: MouseEvent| {
        onclick.emit(get_text(event));
    });
    html!(
        <a {class} {onclick} style={"text-decoration:none"}>{title}</a>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct AsProps {
    pub items: Vec<AProps>
}

#[function_component]
pub fn As(props: &AsProps) -> Html {
    let items = props.items.clone();
    html!(
        <>
        {
            for items.iter().map(|item|{
                html!(
                    <a>{item.title.clone()}</a>
                )
            })
        }
        </>
    )
}

fn get_text(event: MouseEvent) -> AttrValue {
    let event: Event = event.dyn_into().unwrap_throw();
    let event = event.target().unwrap_throw();
    let target: HtmlElement = event.dyn_into().unwrap_throw();
    AttrValue::from(target.inner_text())
}