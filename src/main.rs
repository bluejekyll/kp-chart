use std::fmt::{self, Display, Formatter};
use std::iter::*;
use std::slice::Iter;

fn main() {
    println!("Welcome to Kitchen Patrol Charts");

    let mut jobs = Vec::<Job>::new();
    jobs.push(Job {
        name: "Breakfast dishes",
        people: vec![Ability::Teen, Ability::Child],
    });
    jobs.push(Job {
        name: "Lunch preparation",
        people: vec![Ability::Adult, Ability::Adult],
    });
    jobs.push(Job {
        name: "Lunch dishes",
        people: vec![Ability::Adult, Ability::Teen],
    });
    jobs.push(Job {
        name: "Dinner Setting",
        people: vec![Ability::Teen, Ability::Child, Ability::Child],
    });
    jobs.push(Job {
        name: "Dinner shopping and chef",
        people: vec![Ability::Adult, Ability::Adult],
    });
    jobs.push(Job {
        name: "Dinner dishes",
        people: vec![Ability::Adult, Ability::Teen],
    });
    jobs.push(Job {
        name: "Late night dishes",
        people: vec![Ability::Teen],
    });
    jobs.push(Job {
        name: "Cabin cleanup",
        people: vec![Ability::Adult],
    });
    jobs.push(Job {
        name: "Nag",
        people: vec![Ability::Adult],
    });
    let jobs = jobs;

    let mut people = Vec::<Person>::new();
    // people.push(Person{name: "Nana", ability: Ability::Adult});
    people.push(Person {
        name: "Papa",
        ability: Ability::Adult,
    });
    people.push(Person {
        name: "Chris",
        ability: Ability::Adult,
    });
    people.push(Person {
        name: "Kim",
        ability: Ability::Adult,
    });
    people.push(Person {
        name: "Becky",
        ability: Ability::Adult,
    });
    people.push(Person {
        name: "Carl",
        ability: Ability::Adult,
    });
    people.push(Person {
        name: "Ben",
        ability: Ability::Adult,
    });
    // people.push(Person{name: "Lyndsey", ability: Ability::Adult});
    people.push(Person {
        name: "Big Jake",
        ability: Ability::Adult,
    });
    // people.push(Person{name: "Saeng", ability: Ability::Adult});
    people.push(Person {
        name: "Anna",
        ability: Ability::Teen,
    });
    people.push(Person {
        name: "Luke",
        ability: Ability::Teen,
    });
    people.push(Person {
        name: "Little Jake",
        ability: Ability::Teen,
    });
    people.push(Person {
        name: "Catherine",
        ability: Ability::Child,
    });
    people.push(Person {
        name: "Owen",
        ability: Ability::Child,
    });
    // people.push(Person{name: "Adelise", ability: Ability::Child});
    let people = people;
    let children = people
        .clone()
        .into_iter()
        .filter(|p| p.ability == Ability::Child)
        .collect::<Vec<Person>>();
    let mut children_iter = children.iter().cycle();

    let teens = people
        .clone()
        .into_iter()
        .filter(|p| p.ability == Ability::Teen)
        .collect::<Vec<Person>>();
    let mut teens_iter = teens.iter().cycle();

    let adults = people
        .clone()
        .into_iter()
        .filter(|p| p.ability == Ability::Adult)
        .collect::<Vec<Person>>();
    let mut adults_iter = adults.iter().cycle();

    // make sure we have a good balance of jobs across adults, we nee the count of adult jobs
    let adult_job_count = jobs.iter().fold(0_usize, |count, j| {
        j.people.iter().filter(|a| **a == Ability::Adult).count() + count
    });

    for i in 0..5 {
        let day = Day::new(
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
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Job {
    name: &'static str,
    people: Vec<Ability>,
}

impl Display for Job {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Person {
    name: &'static str,
    ability: Ability,
}

impl Display for Person {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Ability {
    Adult,
    Teen,
    Child,
}

#[derive(Clone, Debug)]
struct Chart {
    weeks: Vec<Day>,
}

#[derive(Clone, Debug)]
struct Day {
    jobs: Vec<(Job, Vec<Person>)>,
}

impl Day {
    fn new(
        jobs: Vec<Job>,
        children: &mut Cycle<Iter<Person>>,
        teens: &mut Cycle<Iter<Person>>,
        adults: &mut Cycle<Iter<Person>>,
    ) -> Self {
        let mut day_jobs = jobs
            .clone()
            .into_iter()
            .map(|j| (j, Vec::<Person>::new()))
            .collect::<Vec<_>>();

        // pass through all children jobs first
        for (job, ref mut workers) in day_jobs.iter_mut() {
            for _ in job
                .people
                .iter()
                .filter(|ability| **ability == Ability::Child)
            {
                workers.push(children.next().expect("ran out of children").clone());
            }
        }
        // println!("current jobs for children: {:#?}", day_jobs);

        // pass through all teens
        for (job, ref mut workers) in day_jobs.iter_mut() {
            for _ in job
                .people
                .iter()
                .filter(|ability| **ability == Ability::Teen)
            {
                workers.push(teens.next().expect("ran out of teens").clone());
            }
        }
        // println!("current jobs for teens: {:#?}", day_jobs);

        // pass through all adults
        for (job, ref mut workers) in day_jobs.iter_mut() {
            for _ in job
                .people
                .iter()
                .filter(|ability| **ability == Ability::Adult)
            {
                workers.push(adults.next().expect("ran out of adults").clone());
            }
        }
        // println!("current jobs for adults: {:#?}", day_jobs);

        Self { jobs: day_jobs }
    }
}

impl Display for Day {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        for (job, people) in self.jobs.iter() {
            write!(fmt, "{}: ", job)?;
            for person in people.iter() {
                write!(fmt, "{}, ", person)?;
            }
            writeln!(fmt, "")?;
        }
        Ok(())
    }
}
