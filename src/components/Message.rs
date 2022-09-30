use std::fmt::Display;

use yew::{prelude::*, virtual_dom::AttrValue};

pub struct Message;

#[derive(Clone, PartialEq, Properties)]
pub struct MessageProps {
    pub text: AttrValue,
    pub objId: AttrValue,
}

impl Component for Message {
    type Message = ();

    type Properties = MessageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="message">
                <div class="objid">{ String::from("#") + &ctx.props().objId }</div>
                <div class="text">{ &ctx.props().text }</div>
            </div>
        }
    }
}
