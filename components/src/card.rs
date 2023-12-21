use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CardProps {
    pub html: Html,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let html = props.html.clone();
    html!(
        <div class="card">
            <div class="card-body">
                {html}
            </div>
        </div>
    )
}