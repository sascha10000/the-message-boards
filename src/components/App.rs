use crate::components::MessageInput::*;
use crate::components::Messages::*;
use crate::services::*;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

use super::Message::MessageProps;

#[function_component(App)]
pub fn app() -> Html {
    let messages = use_state(|| MessageService::get());
    
    let on_post_message = {
        let messages = messages.clone();
        
        Callback::from(move |message: String| {
            let mut c_messages = (*messages).clone();
            c_messages.push(MessageProps { text: AttrValue::from(message), obj_id: AttrValue::from("2") });
            messages.set(c_messages);
        })
    };

    html! {
        <div class="app">
                <div class="title">
                    { "tmb." }
                </div>
                <div class="center">
                    <div class="puffer">{""}</div>
                    <div class="body">
                        <div><Messages messages={(*messages).clone()}></Messages></div>
                        <div><MessageInput {on_post_message}></MessageInput></div>
                    </div>
                    <div class="puffer">{""}</div>
                </div>
            </div>
    }
}
