use gloo::timers::callback::Interval;
use yew::prelude::*;

pub enum Msg {
    AlertStop,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: AttrValue,
    pub callback: Callback<bool>,
}

pub struct Alert {
    timer: Option<Interval>,
    text: AttrValue,
}

impl Component for Alert {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            timer: Some({
                let link = ctx.link().clone();
                Interval::new(1_000, move || link.send_message(Msg::AlertStop))
            }),
            text: ctx.props().text.to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AlertStop => {
                self.timer = None;
                ctx.props().callback.emit(false);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if self.timer.is_some() {
            html!(
                <div class={"alert-c"}>
                    <h1>{&self.text}</h1>
                </div>
            )
        } else {
            html!()
        }
    }
}