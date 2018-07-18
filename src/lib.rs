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
pub struct Chart {
    weeks: Vec<Day>,
}

#[derive(Clone, Debug)]
pub struct Day {
    jobs: Vec<(Job, Vec<Person>)>,
}

impl Day {
    pub fn new(
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
