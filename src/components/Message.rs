use serde::{Serialize, ser::SerializeStruct, Deserialize, de::Visitor};
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

impl Serialize for MessageProps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
            let mut state = serializer.serialize_struct("MessageProps", 2)?;
            state.serialize_field("text", &self.text.into_string());
            state.serialize_field("obj_id", &self.obj_id.into_string());
            state.end()
    }
}

impl Deserialize for MessageProps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(MessageProps {
            text: AttrValue::Static("()"),
            obj_id: AttrValue::Static("()")
        })
    }
}