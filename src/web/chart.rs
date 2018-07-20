use std::ops::DerefMut;

use yew::prelude::*;

use kp_chart;
use kp_chart::data::*;
use web::people::PeopleStore;
use web::*;

#[derive(Clone)]
pub struct Chart {
    week: Week,
}

impl Component<Context> for Chart {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        let jobs = kp_chart::default_jobs();
        let people = PeopleStore::restore(context.deref_mut())
            .map(|p| p.people)
            .unwrap_or_else(kp_chart::default_people);
        let week = kp_chart::calculate(5, jobs, people);

        context.console.debug("creating Chart");
        Self { week }
    }

    fn update(&mut self, _msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Chart> for Chart {
    fn view(&self) -> Html<Context, Self> {
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
            <>
                <h2>{"Job Chart"}</h2>
                <table>
                    <thead>
                        <tr><th>{"Job"}</th> { for self.week.days().iter().map(|d| header(d.name())) }</tr>
                    </thead>
                    <tbody>
                        { for self.week.jobs().enumerate().map(|j| job_row(j)) }
                    </tbody>
                </table>
            </>
        }
    }
}
