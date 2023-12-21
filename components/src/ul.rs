use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct UlProps {
    pub lis: Html,
    pub class: AttrValue,
}

#[function_component]
pub fn Ul(props: &UlProps) -> Html {
    html!(
        <ul class={props.class.clone()}>
            {props.lis.clone()}
        </ul>
    )
}