use std::fmt::{self, Display, Formatter};
use std::iter::*;
use std::slice::Iter;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Job {
    name: &'static str,
    people: Vec<Ability>,
}

impl Job {
    pub fn new(name: &'static str, people: Vec<Ability>) -> Self {
        Self { name, people }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn people(&self) -> &[Ability] {
        &self.people
    }
}

impl Display for Job {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}

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

pub fn calculate_day_jobs() -> Week {
    let jobs = default_jobs();

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
    // people.push(Person{name: "Adelise", ability: Ability::Child});
    let people = people;
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

    let mut days = Vec::with_capacity(5);
    for i in 0..5 {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Person {
    name: &'static str,
    ability: Ability,
}

impl Person {
    pub fn new(name: &'static str, ability: Ability) -> Self {
        Self { name, ability }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn ability(&self) -> Ability {
        self.ability
    }
}

impl Display for Person {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Ability {
    Adult,
    Teen,
    Child,
}

#[derive(Clone, Debug)]
pub struct Day {
    name: String,
    jobs: Vec<(Job, Vec<Person>)>,
}

impl Day {
    pub fn new(
        name: String,
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
            for ability in job.people.iter() {
                match *ability {
                    Ability::Child => {
                        workers.push(children.next().expect("ran out of children").clone())
                    }
                    Ability::Teen => workers.push(teens.next().expect("ran out of teens").clone()),
                    Ability::Adult => {
                        workers.push(adults.next().expect("ran out of adults").clone())
                    }
                }
            }
        }

        Self {
            name,
            jobs: day_jobs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn jobs(&self) -> &[(Job, Vec<Person>)] {
        &self.jobs
    }

    pub fn get_job_people(&self, job: usize) -> &[Person] {
        &self.jobs[job].1
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

#[derive(Clone, Debug)]
pub struct Week {
    week: Vec<Day>,
}

impl Week {
    pub fn new(week: Vec<Day>) -> Self {
        Self { week }
    }

    pub fn num_jobs(&self) -> usize {
        self.week[0].jobs.len()
    }

    pub fn days(&self) -> &[Day] {
        &self.week
    }

    pub fn jobs(&self) -> impl Iterator<Item = &Job> {
        self.week[0].jobs.iter().map(|(job, _)| job)
    }
}
