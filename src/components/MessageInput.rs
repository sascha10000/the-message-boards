use web_sys::{InputEvent, MouseEvent};
use yew::{function_component, html, use_state, Callback, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct MessageInputProps {
    #[prop_or_default(Callback::from(|_| ()))]
    pub on_post_message: Callback<String>,
}

#[function_component(MessageInput)]
pub fn message_input(props: &MessageInputProps) -> Html {
    let text = use_state(|| String::from(""));
    let oninput = {
        let text = text.clone();
        Callback::from(move |input: InputEvent| {
            match input.data() {
                Some(value) => {
                    text.set((*text).clone() + &value)
                },
                None => text.set(String::from("")) // TODO: <BACKSPACE> <ENTF> <ENTER> needs to be handled
            };
        })
    };

    let onclick = {
        let on_post_message = props.on_post_message.clone();
        let text = text.clone();
        Callback::from(move |e: MouseEvent| {
            on_post_message.emit(String::from((*text).clone()));
            text.set(String::from(""));
        }) 
    };

    html! {
        <div class="message-input">
            <textarea maxlength="200" rows="7" value={(*text).clone()} {oninput}></textarea>
            <button {onclick}>{"Post"}</button>
        </div>
    }
}
