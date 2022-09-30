use yew::prelude::*;
use crate::components::Messages::*;

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Messages></Messages>
        }
    }
}