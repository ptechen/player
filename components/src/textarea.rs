// use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use web_sys::{Event, HtmlTextAreaElement};
// use yew::prelude::*;
//
// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     pub value: String,
//     pub label: String,
//     pub style: String,
//     pub onchange: Option<Callback<AttrValue>>,
// }
//
// fn get_value_from_textarea_event(e: Event) -> AttrValue {
//     let event: Event = e.dyn_into().unwrap_throw();
//     let event_target = event.target().unwrap_throw();
//     let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
//     AttrValue::from(target.value())
// }
//
// /// Controlled Textarea Component
// #[function_component]
// pub fn Textarea(props: &Props) -> Html {
//     let Props { value, label, style: _, onchange } = props.clone();
//     let line_no:Vec<&str> = value.split("\n").collect();
//     if onchange.is_some() {
//         let onchange = onchange.unwrap();
//         let onchange = Callback::from(move |event: Event| {
//             onchange.emit(get_value_from_textarea_event(event));
//         });
//         html! (
//                 <div class="flex column">
//                     <div style="line-height:1.5rem">{label}</div>
//                     <div class="flex">
//                         <div class="line_no">
//                         {
//                             for line_no.iter().enumerate().map(|(idx, _)|{
//                                 html!(<p>{idx + 1}</p>)
//                             })
//                         }
//                         </div>
//                         <textarea style="font-size: 15px;line-height: 1.5rem;" {onchange} {value}></textarea>
//                     </div>
//                 </div>
//         )
//     } else {
//         html!(
//                 <div class="flex column">
//                     <div style="line-height:1.5rem">{label}</div>
//                     <div class="flex">
//                         <div class="line_no">
//                         {
//                             for line_no.iter().enumerate().map(|(idx, _)|{
//                                 html!(<p>{idx + 1}</p>)
//                             })
//                         }
//                         </div>
//                         <textarea style="font-size: 15px;line-height: 1.5rem;" {value}></textarea>
//                     </div>
//                 </div>
//         )
//     }
//
// }
