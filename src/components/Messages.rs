use yew::prelude::*;
use super::Message::*;

#[derive(Properties, PartialEq)]
pub struct MessagesProps {
    pub messages: Vec<MessageProps>,
}

#[function_component(Messages)]
pub fn messages(props: &MessagesProps) -> Html {
    html! {
        <div class="messages">
            {
                props.messages.iter().map(|message| html!{
                    <Message text={message.to_owned().text} obj_id={message.to_owned().obj_id}></Message>
                }).collect::<Html>()
            }
            </div>
    }
}