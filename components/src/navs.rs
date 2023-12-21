use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props{
    pub items: Vec<Item>
}

#[derive(Clone, PartialEq, Properties)]
pub struct Item {
    pub title: AttrValue,
    pub href: AttrValue,
}

#[function_component]
pub fn Navs(props: &Props) -> Html {
    html!(
        <ul class="nav flex-column" id="navbarNav">
        {
            for props.items.iter().map(|item|{
                html!(
                    <li>
                        <a class="bd-links-link d-inline-block rounded" href={&item.href}>{&item.title}</a>
                    </li>
                )
            })
        }
        </ul>
    )
}