use yew::{prelude::*, virtual_dom::VChild};

use crate::services::*;

use super::Message::*;

pub struct Messages {
    messages: Vec<MessageProps>
}

impl Component for Messages {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            messages: MessagesService::getMessages()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="messages">
            {
                self.messages.iter().map(|message| html!{
                    <Message text={message.to_owned().text} objId={message.to_owned().objId}></Message>
                }).collect::<Html>()
            }
            </div>
        }
    }
}
