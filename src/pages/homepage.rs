use yew::prelude::*;

use crate::pages::content::Content;

use yew::{
    prelude::*,
    services::{
        ConsoleService,
    },
};


pub enum Msg {
    AddOne,
    InputText(String),
}

pub struct HomePage {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    message: String,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::info("this is homepage..........");
        Self {
            link,
            value: 0,
            message: String::from("initial message"),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
            //     input.focus();
            // }

            ConsoleService::info("this is first render homepage.....");
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::InputText(data) => {
                self.message = data;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div
                style="
                    text-align: center;
                    height: 1000px;
                    background: #2D2D2D;
                "
                class="text-big"
            >
                
                <Content message={self.message.clone()}/>

                <div
                    class="input-group mb-3"
                    style="
                        margin: auto;
                        width: 400px;
                    "
                >
                    

                
                 
                </div>
            </div>
        }
    }
}
