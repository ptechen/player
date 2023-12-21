use bip39::{Language, MnemonicType};
use yew::prelude::*;

pub enum Msg {
    CreateWallet,
    VerifyWallet,
    SetPassword,
    CreateFinish,
}

pub struct AddWallet {
    pub step: u8,
    pub phrase: String,
}

impl Component for AddWallet {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { step: 0, phrase: "".to_string() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateWallet => {
                self.step = 1;
                self.phrase = bip39::Mnemonic::new(MnemonicType::Words12, Language::English).to_string();
                true
            }
            Msg::VerifyWallet => { true }
            Msg::CreateFinish => { true }
            Msg::SetPassword => { true }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.step {
            0 => {
                let onclick = ctx.link().callback(|_| Msg::CreateWallet);
                html!(
                    <div class="create_wallet">
                        <button type="button" class="btn btn-primary" {onclick}>{"创建钱包"}</button>
                    </div>
                )
            }
            1 => {
                let items:Vec<&str> = self.phrase.split(" ").collect();

                html!(
                    <div class="text-center">
                        {
                            for items.iter().step_by(2).map(|item|{

                            })
                        }
                        {
                            for items.iter().enumerate().map(|(idx, item)|{
                                html!(
                                    <div>{idx + 1}{": "}{item}</div>
                                )
                            })
                        }
                    </div>
                )
            }
            2 => html!(),
            _ => html!()
        }
    }
}
