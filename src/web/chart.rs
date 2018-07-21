use yew::prelude::*;

use kp_chart;
use kp_chart::data::*;
use web::people::PeopleStore;
use web::*;

#[derive(Clone)]
pub struct Chart {
    people_version: usize,
    week: Week,
}

#[derive(Clone, Default, PartialEq)]
pub struct ChartProps {
    pub people_version: usize,
}

impl Chart {
    fn calculate(context: &mut Context) -> Self {
        context.console.debug("calculating new week");
        let jobs = kp_chart::default_jobs();
        context.console.debug("====> jobs...");
        let (people_version, people) = PeopleStore::restore(&mut *context)
            .map(|p| (p.inc, p.people))
            .unwrap_or_else(|| (0, kp_chart::default_people()));
        context.console.debug("====> week...");
        Self {
            people_version: people_version,
            week: kp_chart::calculate(5, jobs, people),
        }
    }
}

impl Component<Context> for Chart {
    type Message = ();
    type Properties = ChartProps;

    fn create(_props: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        context.console.debug("creating Chart");

        Self::calculate(&mut *context)
    }

    fn update(&mut self, _msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }

    fn change(
        &mut self,
        props: Self::Properties,
        context: &mut Env<Context, Self>,
    ) -> ShouldRender {
        if self.people_version != props.people_version {
            context.console.debug("updating Chart");
            *self = Self::calculate(&mut *context);
            true
        } else {
            false
        }
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
