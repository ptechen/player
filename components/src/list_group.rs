use yew::prelude::*;
#[derive(Clone, PartialEq, Properties)]
pub struct ListGroupProps {
    pub lis: Vec<Html>,
}

#[function_component]
pub fn ListGroup(props: &ListGroupProps) -> Html {
    html!(
        <ul class="flex-ul">
            {
                for props.lis.iter().map(|item| {
                    html!(<li class="list-group-item">{item.clone()}</li>)
                })
            }
        </ul>
    )
}