use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: AttrValue,
    pub placeholder: AttrValue,
    pub event: Callback<AttrValue>,
    pub input_type: AttrValue,
    pub on_event: OnEvent,
}

#[derive(Clone, PartialEq)]
pub enum OnEvent {
    OnChange,
    OnFocus,
    OnBlur,
    None,
}

fn get_value_from_input_event(e: InputEvent) -> AttrValue {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    // web_sys::console::log_1(&target.value().into());
    AttrValue::from(target.value())
}

fn get_value_from_focus_event(e: FocusEvent) -> AttrValue {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    // web_sys::console::log_1(&target.value().into());
    AttrValue::from(target.value())
}

/// Controlled Text Input Component
#[function_component]
pub fn Input(props: &Props) -> Html {
    let Props { value, placeholder, event, input_type, on_event } = props.clone();
    match on_event {
        OnEvent::OnChange => {
            let oninput = Callback::from(move |input_event: InputEvent| {
                event.emit(get_value_from_input_event(input_event));
            });
            html! {
                <input class="form-control" type={input_type} {placeholder} {value} {oninput} />
            }
        }
        OnEvent::OnFocus => {
            let onfocus = Callback::from(move |focus_event: FocusEvent| {
                event.emit(get_value_from_focus_event(focus_event));
            });
            html! {
                <input class="form-control" type={input_type} {placeholder} {value} {onfocus} />
            }
        }
        OnEvent::OnBlur => {
            let onblur = Callback::from(move |focus_event: FocusEvent| {
                event.emit(get_value_from_focus_event(focus_event));
            });
            html! {
                <input class="form-control" type={input_type} {placeholder} {value} {onblur} />
            }
        }
        OnEvent::None => {
            html! {
                <input class="form-control" type={input_type} {placeholder} {value}/>
            }
        }
    }
}
