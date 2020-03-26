use log::debug;
use yew::prelude::*;

use crate::data::*;
use crate::web::people::PeopleStore;
use yew::services::{storage::Area, StorageService};

#[derive(Clone)]
pub struct Chart {
    people_version: usize,
    week: Week,
}

#[derive(Clone, Default, PartialEq, Properties)]
pub struct ChartProps {
    pub people_version: usize,
}

impl Chart {
    fn calculate() -> Self {
        debug!("calculating new week");
        let mut local_store = StorageService::new(Area::Local).expect("failed to get storage");

        let jobs = crate::default_jobs();
        let (people_version, people) = PeopleStore::restore(&mut local_store)
            .map(|p| (p.inc, p.people))
            .unwrap_or_else(|| (0, crate::default_people()));
        Self {
            people_version: people_version,
            week: crate::calculate(5, jobs, people),
        }
    }
}

impl Component for Chart {
    type Message = ();
    type Properties = ChartProps;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        debug!("creating Chart");
        Self::calculate()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.people_version != props.people_version {
            debug!("updating Chart");
            *self = Self::calculate();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let header = |name: &str| {
            html! {
                <th>{ format!("{}", name) }</th>
            }
        };
        let people_cell = |people: &[Person]| {
            let mut people_str = String::new();
            for person in people {
                people_str.push_str(person.name());
                people_str.push_str(", ");
            }

            html! {
                <td>{ people_str }</td>
            }
        };
        let job_row = |(job_idx, job): (usize, &Job)| {
            let days = self.week.days();
            html! {
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
