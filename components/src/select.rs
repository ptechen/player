use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlSelectElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub option: AttrValue,
    pub options: Vec<AttrValue>,
    pub onchange: Callback<AttrValue>,
}

pub struct Select {
    pub option: AttrValue,
}

impl Component for Select {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { option: ctx.props().option.clone() }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let option = ctx.props().option.to_owned();
        let onchange = ctx.props().onchange.to_owned();
        let onchange = Callback::from(move |e: Event| {
            onchange.emit(get_value_from_change_event(e));
        });
        html!(
            <select class="form-select" {onchange}>
                {
                    for ctx.props().options.iter().map(|item|
                        if item == &option {
                            html!{<option value={item.clone()} selected={true}>{item}</option>}
                        } else {
                            html!{<option value={item.clone()} >{item}</option>}
                        }
                    )
                }
            </select>
        )
    }
}

fn get_value_from_change_event(e: Event) -> AttrValue {
    let event_target = e.target().unwrap_throw();
    let target: HtmlSelectElement = event_target.dyn_into().unwrap_throw();
    AttrValue::from(target.selected_index().to_string())
}