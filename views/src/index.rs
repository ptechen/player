use yew::prelude::*;
use crate::add_wallet::AddWallet;

pub enum Msg {
    Update(AttrValue),
    AddWallet((AttrValue, AttrValue))
}

pub struct Index {
    pub content: Html,
    pub phrase: AttrValue,
    pub password: AttrValue,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let event = ctx.link().callback(|v|Msg::AddWallet(v));
        Self { content: html!(<AddWallet {event}/>), phrase: Default::default(), password: Default::default() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(tag) => {
                let tag = tag.to_string();
                let tag = tag.as_str();
                let event = ctx.link().callback(|v|Msg::AddWallet(v));
                self.content = match tag {
                    _ => { html!(<AddWallet event={event}/>) }
                };
                true
            }
            Msg::AddWallet((phrase, password)) => {
                self.phrase = phrase;
                self.password = password;
                self.content = html!(<h1>{&self.phrase}{&self.password}</h1>);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div class="menu">
                <div class="menu-content">
                    {self.content.clone()}
                </div>
            </div>
        )
    }
}
