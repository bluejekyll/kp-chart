use yew::prelude::*;

use kp_chart;
use kp_chart::data::*;
use web::*;

#[derive(Clone)]
pub struct People {
    people: Vec<Person>,
}

impl Component<Context> for People {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating People");

        let people = kp_chart::default_people();
        Self { people: people }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, People> for People {
    fn view(&self) -> Html<Context, Self> {
        let person_row = |person: &Person| {
            html!{
                <tr>
                    <td>{ person.name() }</td>
                    <td>{ person.ability().to_string() }</td>
                    <td><Select: is_selected=true, /></td>
                </tr>
            }
        };

        html! {
            <>
                <h2>{"All the beautiful people"}</h2>
                <table>
                    <tr><th>{"Person"}</th><th>{"Ability"}</th><th>{"Include"}</th></tr>
                    { for self.people.iter().map(|p| person_row(p)) }
                </table>
            </>
        }
    }
}

struct Select {
    is_selected: bool,
}

#[derive(Clone, Eq, PartialEq, Default)]
struct SelectOpts {
    is_selected: bool,
}

impl Component<Context> for Select {
    type Message = ();
    type Properties = SelectOpts;

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        Self {
            is_selected: props.is_selected,
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Select> for Select {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <><input></input></>
        }
    }
}
