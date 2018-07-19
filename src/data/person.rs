use std::fmt::{self, Display, Formatter};

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

impl Display for Ability {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Ability::Adult => write!(fmt, "Adult"),
            Ability::Teen => write!(fmt, "Teen"),
            Ability::Child => write!(fmt, "Child"),
        }
    }
}
