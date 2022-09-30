use yew::virtual_dom::AttrValue;

use crate::components::Message::*;

pub struct MessagesService;

impl MessagesService {
    pub fn new() -> Self {
        Self
    }
    pub fn getMessages() -> Vec<MessageProps> {
        vec![MessageProps {
            text: AttrValue::from("Message 1"),
            objId: AttrValue::from("123"),
        },
        MessageProps {
            text: AttrValue::from("Message 2"),
            objId: AttrValue::from("456"),
        },
        MessageProps {
            text: AttrValue::from("Message 3"),
            objId: AttrValue::from("789"),
        }]
    }
}