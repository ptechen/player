use gloo::console::log;
use web_sys::HtmlElement;
use yew::prelude::*;

use gloo::events::EventListener;

#[function_component]
pub fn MyComponent() -> Html {
    let div_node_ref = use_node_ref();

    use_effect_with(div_node_ref.clone(),
        {
            let div_node_ref = div_node_ref.clone();
            log!("1143");
            move |_| {
                let mut custard_listener = None;

                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    // Create your Callback as you normally would
                    let oncustard = Callback::from(move |_: Event| {
                        log!("1123");
                        // do something about custard..
                    });

                    // Create a Closure from a Box<dyn Fn> - this has to be 'static
                    let listener = EventListener::new(
                        &element,
                        "custard",
                        move |e| oncustard.emit(e.clone())
                    );

                    custard_listener = Some(listener);
                }

                move || drop(custard_listener)
            }
        },
    );

    html! {
        <div ref={div_node_ref} id="my-div">{"123"}</div>
    }
}