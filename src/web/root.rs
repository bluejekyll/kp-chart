use log::debug;
use yew::prelude::*;

use crate::web::*;

pub struct RootModel {
    people_version: usize,
    link: ComponentLink<Self>,
}

pub enum RootMsg {
    PeopleUpdated(usize),
}

impl Component for RootModel {
    // Some details omitted. Explore the examples to get more.

    type Message = RootMsg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RootModel {
            people_version: 0,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMsg::PeopleUpdated(version) => {
                debug!("root people version: {}", version);
                if self.people_version != version {
                    self.people_version = version;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{"Kitchen Patrol Charts"}</h1>
                <Chart people_version=self.people_version />
                <PeopleModel on_save=self.link.callback(|inc| RootMsg::PeopleUpdated(inc)),/>
            </div>
        }
    }
}
