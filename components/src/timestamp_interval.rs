use chrono::Utc;
use gloo::timers::callback::Interval;
use yew::prelude::*;

pub enum Msg {
    UpdateTime,
    StartTime,
    StopTime,
    CopyTimestamp
}

pub struct TimestampInterval {
    current_time: String,
    timestamp_timer: Option<Interval>,
    status: u8,
}

impl Component for TimestampInterval {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let timestamp_timer = {
            let link = ctx.link().clone();
            Interval::new(1_000, move || link.send_message(Msg::UpdateTime))
        };
        Self {
            timestamp_timer: Some(timestamp_timer),
            current_time: Utc::now().timestamp().to_string(),
            status: 1,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => {
                self.current_time = Utc::now().timestamp().to_string();
                true
            }
            Msg::StartTime => {
                let timestamp_timer = {
                    let link = ctx.link().clone();
                    Interval::new(1_000, move || link.send_message(Msg::UpdateTime))
                };
                self.timestamp_timer = Some(timestamp_timer);
                self.status = 1;
                true
            }
            Msg::StopTime => {
                self.timestamp_timer = None;
                self.status = 0;
                true
            }
            Msg::CopyTimestamp => {
                // let data = JsValue::from_str(&self.current_time);
                // let document = web_sys::window()
                //     .expect_throw("window is undefined")
                //     .document()
                //     .expect_throw("document is undefined");
                // let func = document.;
                // log!(document);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick_start_time = ctx.link().callback(|_|Msg::StartTime);
        let onclick_stop_time = ctx.link().callback(|_| Msg::StopTime);
        let onclick_timestamp = ctx.link().callback(|_|Msg::CopyTimestamp);
        html!(
                <div class="flex">
                    <div class="flex-1 align-self-center">
                        <button type="button" class="custom-btn btn-9">{"现在"}</button>
                    </div>
                    <div class="flex-1 align-self-center">
                        <button type="button" class="custom-btn btn-11" onclick={onclick_timestamp} >{self.current_time.clone()}</button>
                    </div>
                    <div class="flex-1 align-self-center">
                        if self.status == 0 {
                            <button type="button" class="custom-btn btn-9" onclick={onclick_start_time}>{"开始"}</button>
                        } else {
                            <button type="button" class="custom-btn btn-9" onclick={onclick_stop_time}>{"停止"}</button>
                        }
                    </div>
                </div>

        )
    }
}