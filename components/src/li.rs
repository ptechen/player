use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LiProps{
    pub class: AttrValue,
    pub html: Html
}

#[function_component]
pub fn Li(props: &LiProps) -> Html {
    let class = props.class.clone();
    let html = props.html.clone();
    html!(
        <li class={class}>
            {html}
        </li>
    )
}