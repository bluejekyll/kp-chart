use crate::data::{Day, Job};

#[derive(Clone, Debug)]
pub struct Week {
    week: Vec<Day>,
}

impl Week {
    pub fn new(week: Vec<Day>) -> Self {
        Self { week }
    }

    pub fn num_jobs(&self) -> usize {
        self.week[0].jobs().len()
    }

    pub fn days(&self) -> &[Day] {
        &self.week
    }

    pub fn jobs(&self) -> impl Iterator<Item = &Job> {
        self.week[0].jobs().iter().map(|(job, _)| job)
    }
}
