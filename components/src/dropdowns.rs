use yew::prelude::*;
use crate::a::A;

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownsProps {
    pub title: AttrValue,
    pub ul: UlProps,
    pub callback: Callback<AttrValue>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UlProps{
    pub lis: Vec<LiProps>
}

#[derive(Clone, PartialEq, Properties)]
pub struct LiProps {
    pub title: AttrValue
}

#[function_component]
pub fn Dropdowns(props: &DropdownsProps) -> Html {
    html!(
        <div class="btn-group">
            <button class="btn btn-success dropdown-toggle dropdown-toggle-split" type={"button"} data-bs-toggle={"dropdown"} aria-expanded={"false"}>
                {&props.title}
            </button>
            <ul class="dropdown-menu">
                {
                    for props.ul.lis.iter().map(|item|{
                        html!(<li><A onclick={&props.callback} class={"dropdown-item"} title={&item.title}/></li>)
                    })
                }
            </ul>
        </div>
    )
}

impl DropdownsProps {
    pub fn to_html(&self) -> Html {
        html!(<Dropdowns title={&self.title}  ul={self.ul.clone()} callback={&self.callback}/>)
    }
}
