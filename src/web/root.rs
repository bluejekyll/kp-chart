use yew::prelude::*;

use kp_chart::*;
use web::*;

pub struct RootModel {}

impl Component<Context> for RootModel {
    // Some details omitted. Explore the examples to get more.

    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        RootModel {}
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, RootModel> for RootModel {
    fn view(&self) -> Html<Context, Self> {
        use web::Chart;

        html! {
            <h1>{"Welcome to Kitchen Patrol Charts"}</h1>
            <Chart: />
        }
    }
}
