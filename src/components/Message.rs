use yew::{prelude::*, virtual_dom::AttrValue};

#[derive(Clone, PartialEq, Properties)]
pub struct MessageProps {
    pub text: AttrValue,
    pub obj_id: AttrValue,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    html! {
        <div class="message">
            <div class="objid">{ String::from("#") + &props.obj_id }</div>
            <div class="text">{ &props.text }</div>
        </div>
    }
}