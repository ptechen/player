use yew::prelude::*;
use crate::li::LiProps;
use crate::li::Li;

#[derive(Clone, PartialEq, Properties)]
pub struct LisProps{
    pub lis: Vec<LiProps>
}

#[function_component]
pub fn Lis(props: &LisProps) -> Html {
    let lis = props.lis.clone();
    html!(
        {
            for lis.iter().map(|item|{
                html!(<Li html={item.html.clone()} class={&item.class}/>)
            })
        }
    )
}