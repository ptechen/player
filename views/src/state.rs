use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    pub phrase: AttrValue,
    pub password: AttrValue,
}

