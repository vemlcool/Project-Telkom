use yew::prelude::*;

pub enum Msg {
    AddOne,
}


#[derive(Clone)]
pub struct Superhero {
    name: String,
    age: u8,
}
impl Default for Superhero {
    fn default() -> Superhero {
        Superhero{
            name: String::from("batman"),
            age: 35
        }
    }
}

fn get_message() -> String {
    String::from("message from function")
}


#[derive(Properties, Clone)]
pub struct ContentProps {
    // #[prop_or_default]
    // #[prop_or(String::from("this is value"))]
    #[prop_or_else(get_message)]
    pub message: String,
    #[prop_or_default]
    pub superhero: Superhero,
}


pub struct Content {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    // link: ComponentLink<Self>,
    value: i64,
    props: ContentProps,
}

impl Component for Content {
    type Message = Msg;
    type Properties = ContentProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            value: 0,
            props,
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
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        if self.props.message != props.message {
            self.props.message = props.message;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {

            <div
            
            style="
                text-align:center;
                right: 200px;
                height: 360px;
                color: white;
                background: darkgray;
            "
            class="text-big"
        >
            { "register"}
           
                    

                    <div
                        class="input-group mb-3"
                        style="
                            margin: auto;
                            width: 350px;
                        "
                    >
                    
                        <input
                            type="text"
                            class="form-control"
                            placeholder="email"
                            aria-label="Email"
                            aria-describedby="basic-addon1"
                        
                        />
                    </div><div
                    class="input-group mb-3"
                    style="
                        margin: auto;
                        width: 350px;
                    "
                >
                
                    <input
                        type="text"
                        class="form-control"
                        placeholder="username"
                        aria-label="Username"
                        aria-describedby="basic-addon1"
                    
                    />
                </div><div
                class="input-group mb-3"
                style="
                    margin: auto;
                    width:350px;
                "
            >
            
                <input
                    type="text"
                    class="form-control"
                    placeholder="password"
                    aria-label="Password"
                    aria-describedby="basic-addon1"
                
                />
            </div><div
            class="input-group mb-3"
            style="
                margin: auto;
                width: 350px;
            "
        >
        
            <input
                type="text"
                class="form-control"
                placeholder="re-type password"
                aria-label="re-type password"
                aria-describedby="basic-addon1"
            
            />
        </div>

        
                // disabled={true}
        

            <button
                type="button"
                class="btn btn-primary"
                style="
                
                width: 120px;
                color: black;
                background:#59CDA3;

            "
               
            >
                { "register" }
            </button>



        </div>


        }
    }
}
