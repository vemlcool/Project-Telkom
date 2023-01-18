use yew::{
    format::{ Json, Nothing },
    prelude::*,
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
        ConsoleService,
    },
};
use serde::{
    Deserialize,
    Serialize,
};


#[derive(Deserialize, Debug, Clone)]
pub struct Schedule {
    task: String,
    superhero: String,
    is_on_going: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SchedulesData {
    list: Option<Vec<Schedule>>,
    world: Option<String>,
    error_description: Option<String>,
    other_data: String,
}




pub enum Msg {
    RequestData,
    GetData(SchedulesData),
    ResponseError(String),
}

pub struct Schedules {
    fetch_task: Option<FetchTask>,
    link: ComponentLink<Self>,
    schedules: Vec<Schedule>,
    error: Option<String>,
}

impl Component for Schedules {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            fetch_task: None,
            link,
            schedules: vec![],
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RequestData => {


                // FETCHING....

                let request = Request::get("http://localhost:3000/schedules")
                    // .header("access_token", get_access_token().unwrap_or_default())
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = 
                    self.link.callback(|response: Response<Json<Result<SchedulesData, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        let status_number = meta.status.as_u16();

                        match data {
                            Ok(dataok) => {
                                ConsoleService::info(&format!("data response {:?}", &dataok));

                                if status_number == 200 {
                                    Msg::GetData(dataok)
                                } else {
                                    Msg::ResponseError(String::from("status bukan 200"))
                                }

                            }
                            Err(error) => {
                                // ConsoleService::info("kondisi error dari server mati");
                                Msg::ResponseError(error.to_string())
                            }
                        }
                    });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);


                true
            }
            Msg::GetData(data) => {
                ConsoleService::info(&format!("data is {:?}", data));
                self.schedules = data.list.unwrap_or_default();
                true
            }
            Msg::ResponseError(text) => {
                ConsoleService::info(&format!("error is {:?}", text));
                self.error = Some(text);
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // self.link.send_message(Msg::RequestData);
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
          
        
    }
}
}