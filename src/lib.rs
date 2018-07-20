#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod data;

use std::fmt::{self, Display, Formatter};
use std::iter::*;
use std::slice::Iter;

use data::*;

pub fn default_jobs() -> Vec<Job> {
    let mut jobs = Vec::<Job>::new();
    jobs.push(Job::new(
        "Breakfast dishes",
        vec![Ability::Teen, Ability::Child],
    ));
    jobs.push(Job::new(
        "Lunch preparation",
        vec![Ability::Adult, Ability::Adult],
    ));
    jobs.push(Job::new(
        "Lunch dishes",
        vec![Ability::Adult, Ability::Teen],
    ));
    jobs.push(Job::new(
        "Dinner Setting",
        vec![Ability::Teen, Ability::Child, Ability::Child],
    ));
    jobs.push(Job::new(
        "Dinner shopping and chef",
        vec![Ability::Adult, Ability::Adult],
    ));
    jobs.push(Job::new(
        "Dinner dishes",
        vec![Ability::Adult, Ability::Teen],
    ));
    jobs.push(Job::new("Late night dishes", vec![Ability::Teen]));
    jobs.push(Job::new("Cabin cleanup", vec![Ability::Adult]));
    jobs.push(Job::new("Nag", vec![Ability::Adult]));

    jobs
}

pub fn default_people() -> Vec<Person> {
    let mut people = Vec::<Person>::new();
    // people.push(Person{name: "Nana", ability: Ability::Adult});
    people.push(Person::new("Papa", Ability::Adult));
    people.push(Person::new("Chris", Ability::Adult));
    people.push(Person::new("Kim", Ability::Adult));
    people.push(Person::new("Becky", Ability::Adult));
    people.push(Person::new("Carl", Ability::Adult));
    people.push(Person::new("Ben", Ability::Adult));
    // people.push(Person{name: "Lyndsey", ability: Ability::Adult});
    people.push(Person::new("Big Jake", Ability::Adult));
    // people.push(Person{name: "Saeng", ability: Ability::Adult});
    people.push(Person::new("Anna", Ability::Teen));
    people.push(Person::new("Luke", Ability::Teen));
    people.push(Person::new("Little Jake", Ability::Teen));
    people.push(Person::new("Catherine", Ability::Child));
    people.push(Person::new("Owen", Ability::Child));

    return people;
}

pub fn calculate_day_jobs() -> Week {
    let jobs = default_jobs();
    let people = default_people();
    calculate(5, jobs, people)
}

pub fn calculate(num_days: usize, jobs: Vec<Job>, people: Vec<Person>) -> Week {
    let children = people
        .clone()
        .into_iter()
        .filter(|p| p.ability() == Ability::Child)
        .collect::<Vec<Person>>();
    let mut children_iter = children.iter().cycle();

    let teens = people
        .clone()
        .into_iter()
        .filter(|p| p.ability() == Ability::Teen)
        .collect::<Vec<Person>>();
    let mut teens_iter = teens.iter().cycle();

    let adults = people
        .clone()
        .into_iter()
        .filter(|p| p.ability() == Ability::Adult)
        .collect::<Vec<Person>>();
    let mut adults_iter = adults.iter().cycle();

    // make sure we have a good balance of jobs across adults, we nee the count of adult jobs
    let adult_job_count = jobs.iter().fold(0_usize, |count, j| {
        j.people().iter().filter(|a| **a == Ability::Adult).count() + count
    });

    let mut days = Vec::with_capacity(num_days);
    for i in 0..num_days {
        let day = Day::new(
            format!("day_{}", i),
            jobs.clone(),
            &mut children_iter,
            &mut teens_iter,
            &mut adults_iter,
        );

        // force an additional rotation to offset Dinner duty
        //   we need to make sure we balance the rotation of major adult jobs
        if (adult_job_count + 1) == adults.len() {
            adults_iter.next();
            adults_iter.next();
        } else {
            adults_iter.next();
        }

        println!("");
        println!("day {}:\n{}", i, day);
        days.push(day);
    }

    Week::new(days)
}
