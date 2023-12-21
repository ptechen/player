use yew::prelude::*;
use crate::li::{Li, LiProps};
use crate::ul::Ul;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub items: Vec<LiProps>,
    pub class: AttrValue,
}


#[function_component]
pub fn Menu(props: &Props) -> Html {
    let items = props.items.clone();
    let html = html!(
        {
            for items.iter().map(|item|{
                html!(<Li class={&item.class} html={item.html.clone()}/>)
            })
        }

    );
    html!(
        <div>
            <Ul lis={html.clone()} class={&props.class}/>
        </div>
    )
}