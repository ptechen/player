use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: AttrValue,
    pub label: AttrValue,
    pub onchange: Callback<AttrValue>,
}

fn get_value_from_textarea_event(e: Event) -> AttrValue {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    AttrValue::from(target.value())
}

/// Controlled Textarea Component
#[function_component]
pub fn Textarea(props: &Props) -> Html {
    let Props {id, label, onchange } = props.clone();
    let onchange = Callback::from(move |event: Event| {
        onchange.emit(get_value_from_textarea_event(event));
    });
    html!(
        <div>
            <textarea {onchange} value={""} id={id}></textarea>
            <label for="floatingTextarea">{label}</label>
        </div>
        )
}
