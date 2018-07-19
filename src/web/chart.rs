use yew::prelude::*;

use kp_chart::*;
use web::*;

#[derive(Clone)]
pub struct Chart {
    week: Week,
}

// #[derive(Clone, Default, Eq, PartialEq)]
// pub struct Props {
//     opt: bool,
// }

// impl Chart {
//     pub fn new(days: Vec<Day>) -> Self {
//         Self { days }
//     }

//     pub fn view(&self) -> Html<Context, RootModel> {
//         html! {
//             <table>
//             <tr>{ for self.days.iter().map(|d| header(d.name())) }</tr>
//             </table>
//         }
//     }
// }

impl Component<Context> for Chart {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let week = calculate_day_jobs();

        context.console.debug("creating Chart");
        Self { week }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Chart> for Chart {
    fn view(&self) -> Html<Context, Self> {
        let jobs = self.week.jobs();
        let header = |name: &str| {
            html!{
                <th>{ format!("{}", name) }</th>
            }
        };
        let people_cell = |people: &[Person]| {
            let mut people_str = String::new();
            for person in people {
                people_str.push_str(person.name());
                people_str.push_str(", ");
            }

            html!{
                <td>{ people_str }</td>
            }
        };
        let job_row = |(job_idx, job): (usize, &Job)| {
            let days = self.week.days();
            html!{
                <tr>{ header(job.name()) } { for days.iter().map(|d| people_cell(d.get_job_people(job_idx))) }</tr>
            }
        };

        html! {
            <table>
                <tr><th>{"Job"}</th> { for self.week.days().iter().map(|d| header(d.name())) }</tr>
                { for self.week.jobs().enumerate().map(|j| job_row(j)) }
            </table>
        }
    }
}

// fn day_header(name: &str) -> Html<Context, Chart> {
//     html! {
//         <th>{ format!("{}", name) }</th>
//     }
// }

// fn job_header(name: &str)
