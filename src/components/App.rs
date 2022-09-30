use crate::components::Messages::*;
use yew::prelude::*;

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="app">
                <div class="title">
                    { "tmb." }
                </div>
                <div class="body">
                    <div class="puffer">{""}</div>
                    <div><Messages></Messages></div>
                </div>
            </div>
        }
    }
}
