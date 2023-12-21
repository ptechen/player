use yew::prelude::*;
use crate::{
    menu::Menu,
};
use crate::add_wallet::AddWallet;
use crate::room_list::RoomList;

pub enum Msg {
    Update(AttrValue)
}

pub struct Index {
    pub content: Html,
}

impl Component for Index {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { content: html!(<AddWallet/>) }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(tag) => {
                let tag = tag.to_string();
                let tag = tag.as_str();
                self.content = match tag {
                    _ => { html!(<AddWallet/>) }
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(Msg::Update);
        html!(
            <div class="menu">
                <div class="menu-content">
                    {self.content.clone()}
                </div>
            </div>
        )
    }
}
