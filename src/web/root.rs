use yew::prelude::*;

use web::*;

pub struct RootModel {
    people_version: usize,
}

pub enum RootMsg {
    PeopleUpdated(usize),
}

impl Component<Context> for RootModel {
    // Some details omitted. Explore the examples to get more.

    type Message = RootMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        RootModel { people_version: 0 }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            RootMsg::PeopleUpdated(version) => {
                context
                    .console
                    .debug(&format!("root people version: {}", version));
                if self.people_version != version {
                    self.people_version = version;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Renderable<Context, RootModel> for RootModel {
    fn view(&self) -> Html<Context, Self> {
        use web::Chart;

        html! {
            <div>
                <h1>{"Kitchen Patrol Charts"}</h1>
                <Chart: people_version={self.people_version},/>
                <PeopleModel: on_save=|inc| RootMsg::PeopleUpdated(inc),/>
            <div/>
        }
    }
}
