use yew::prelude::*;
use components::a::A;
use components::li::{LiProps};
use components::menu;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub onclick: Callback<AttrValue>,
}

#[function_component]
pub fn Menu(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let items = vec![
        LiProps { html: html!(<A title="登录" onclick={onclick.clone()} class={"nav-item"}/>), class: AttrValue::from("list-group-item") },
        LiProps { html: html!(<A title="房间列表" onclick={onclick.clone()} class={"nav-item"}/>), class: AttrValue::from("list-group-item") },
        LiProps { html: html!(<A title="反馈" onclick={onclick.clone()} class={"nav-item"}/>), class: AttrValue::from("list-group-item") },
    ];
    html!(
            <div>
            <menu::Menu {items} class={"list-group"}/>
            </div>
        )
}
