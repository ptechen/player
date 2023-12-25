use std::collections::HashMap;
use bip39::{Language, MnemonicType};
use gloo_console::log;
use yew::prelude::*;
use yewdux::prelude::*;
use components::a::A;
use components::button::Button;
use components::input::{Input, OnEvent};

pub enum Msg {
    CreateWallet,
    VerifyWallet,
    ClickAdd(AttrValue),
    ClickDel(AttrValue),
    SetPassword(AttrValue),
    RePassword(AttrValue),
    Submit,
    CreateFinish,
    Back,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub event: Callback<(AttrValue, AttrValue)>,
}

#[derive(Default)]
pub struct AddWallet {
    pub step: u8,
    pub phrase: AttrValue,
    pub verify_phrase: Vec<AttrValue>,
    pub password: AttrValue,
    pub re_password: AttrValue,
}

impl Component for AddWallet {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { step: 0, phrase: Default::default(), verify_phrase: Default::default(), password: Default::default(), re_password: Default::default() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateWallet => {
                self.step = 1;
                self.phrase = AttrValue::from(bip39::Mnemonic::new(MnemonicType::Words12, Language::English).to_string());
                true
            }
            Msg::VerifyWallet => {
                self.step = 2;
                true
            }
            Msg::ClickAdd(val) => {
                log!(val.as_str());
                if self.verify_phrase.contains(&val) {
                    false
                } else {
                    self.verify_phrase.push(val);
                    if self.verify_phrase.len() == 12 {
                        if self.verify_phrase.join(" ") == self.phrase {
                            self.step += 1;
                        } else {
                            self.step -= 1;
                            self.verify_phrase = Default::default();
                        }
                    }
                    true
                }
            }
            Msg::ClickDel(val) => {
                for (idx, item) in self.verify_phrase.iter().enumerate() {
                    if item == val.as_str() {
                        self.verify_phrase.remove(idx);
                        break;
                    }
                }
                true
            }
            Msg::CreateFinish => { true }
            Msg::SetPassword(password) => {
                self.password = password;
                true
            }
            Msg::RePassword(re_password) => {
                self.re_password = re_password;
                if self.password != self.re_password {
                    false
                } else {
                    true
                }
            }
            Msg::Submit => {
                if !self.password.is_empty() && self.password == self.re_password {
                    let Props { event } = ctx.props().to_owned();
                    let phrase = self.phrase.clone();
                    let password = self.password.clone();
                    event.emit((phrase, password));
                    self.step += 1;
                    false
                } else {
                    true
                }
            }

            Msg::Back => {
                self.step -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.step {
            0 => {
                let onclick = ctx.link().callback(|_| Msg::CreateWallet);
                html!(
                    <div class="container text-center">
                        <div class="row align-items-center new-wallet-body">
                        <div class="col">
                            <div class="row">
                                <div class="col">
                                    <button type="button" class="btn btn-primary fs-2" {onclick}>{"创建钱包"}</button>
                                </div>
                            </div>
                            <div class="row">
                                <div class="col">
                                    <button type="button" class="btn btn-primary fs-2">{"导入钱包"}</button>
                                </div>
                            </div>
                        </div>
                        </div>
                    </div>
                )
            }
            1 => {
                let items: Vec<AttrValue> = self.phrase.split(" ").map(|item| AttrValue::from(item.to_owned())).collect();
                let onclick = ctx.link().callback(|_| Msg::VerifyWallet);
                html!(
                    <div class="container text-center">
                        <div class="row">
                            <h1>{"NewWallet"}</h1>
                        </div>
                        <div class="row align-items-start new-wallet-body">
                        <div class="col border">
                            <div class="row">
                            <h4>{"Please write the mnemonic phrase by hand. It cannot be retrieved if lost."}</h4>
                            </div>
                            {
                                for items.iter().map(|item|{
                                    html!(
                                        <div class="row">
                                            <div class="col"><a class="fs-2 border rounded-3 a">{item}</a></div>
                                        </div>
                                    )
                                })
                            }
                        </div>
                        <div class="row align-items-end">
                            <div class="col">
                                <button type="button" class="btn btn-primary" {onclick}>{"VerifyMnemonic"}</button>
                            </div>
                        </div>
                        </div>
                    </div>
                )
            }
            2 => {
                let items: Vec<AttrValue> = self.phrase.split(" ").map(|item| AttrValue::from(item.to_owned())).collect();
                let mut items_c = items.clone();
                let mut indexes = HashMap::new();
                for (idx, item) in items.iter().enumerate() {
                    indexes.insert(item.to_owned(), idx);
                }
                items_c.sort_by(|a, b| a.len().cmp(&b.len()));
                let back = ctx.link().callback(|_| Msg::Back);
                let onclick = ctx.link().callback(|val| Msg::ClickAdd(val));
                let onclick_del = ctx.link().callback(|val| Msg::ClickDel(val));
                html!(
                    <div class="container text-center">
                        <div class="row">
                            <h1>{"VerifyMnemonic"}</h1>
                        </div>
                        <div class="row align-items-start new-wallet-body">
                        <div class="col border">
                            <div class="row">
                            <h4>{"Click on the mnemonic phrases in the correct order."}</h4>
                            </div>
                            {
                                for items_c.iter().map(|item|{
                                    if self.verify_phrase.contains(item) {
                                        let idx = *indexes.get(item).unwrap_or(&0);
                                        if items.get(idx) == self.verify_phrase.get(idx) {
                                            html!(
                                                <div class="row">
                                                    <div class="col"><A class={"fs-2 border rounded-3 aa"} title={item} onclick={onclick_del.clone()}/></div>
                                                </div>
                                            )
                                        } else {
                                            html!(
                                                <div class="row">
                                                    <div class="col"><A class={"fs-2 border rounded-3 aaa"} title={item} onclick={onclick_del.clone()}/></div>
                                                </div>
                                    )
                                        }
                                    } else {
                                        html!(
                                            <div class="row">
                                                <div class="col"><A class={"fs-2 border rounded-3 a"} title={item} onclick={onclick.clone()}/></div>
                                            </div>
                                        )
                                    }

                                })
                            }
                        </div>
                        <div class="row align-items-end">
                            <div class="col">
                                <button type="button" class="btn btn-primary" onclick={back}>{"Back"}</button>
                            </div>
                        </div>
                        </div>
                    </div>
                )
            }
            3 => {
                let on_event = ctx.link().callback(|password| Msg::SetPassword(password));
                let on_event_re = ctx.link().callback(|password| Msg::RePassword(password));
                let onclick = ctx.link().callback(|_| Msg::Submit);
                html!(
                    <div class="container text-center">
                        <div class="row">
                            <h1>{"SetPassword"}</h1>
                        </div>
                    <div class="row align-items-center new-wallet-body">
                        <div class="col">
                            <div class="row">
                                <div class="col">
                                    <Input value={&self.password} placeholder={"Please set a password."} event={on_event} input_type={"password"} on_event={OnEvent::OnBlur}/>
                                </div>
                            </div>
                            <div class="row">
                                <div class="col">
                                    <Input value={&self.password} placeholder={"Please set a password."} event={on_event_re} input_type={"password"} on_event={OnEvent::OnBlur}/>
                                </div>
                            </div>
                            <div class="row">
                                <div class="col">
                                    <Button {onclick} class={"btn btn-primary"} title={"Submit"}/>
                                </div>
                            </div>
                        </div>
                    </div>
                    </div>
                )
            }
            _ => html!()
        }
    }
}
