use serde::de::Error;
use serde_json::{json, Value};
use yew::virtual_dom::AttrValue;

use crate::components::Message::*;
use std::vec;

pub fn get() -> Vec<MessageProps> {
    vec![
        MessageProps {
            text: AttrValue::from("Message 1"),
            obj_id: AttrValue::from("123"),
        },
        MessageProps {
            text: AttrValue::from("Message 2"),
            obj_id: AttrValue::from("456"),
        },
        MessageProps {
            text: AttrValue::from("Message 3"),
            obj_id: AttrValue::from("789"),
        },
    ]
}
