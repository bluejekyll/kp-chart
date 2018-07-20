use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Person {
    name: String,
    ability: Ability,
}

impl Person {
    pub fn new(name: &'static str, ability: Ability) -> Self {
        Self {
            name: name.to_string(),
            ability,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ability(&self) -> Ability {
        self.ability
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_ability(&mut self, ability: Ability) {
        self.ability = ability;
    }
}

impl Display for Person {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.name)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Ability {
    Adult = 0,
    Teen = 1,
    Child = 2,
}

impl Default for Ability {
    fn default() -> Self {
        Ability::Adult
    }
}

impl Ability {
    pub fn enumerate() -> &'static [Ability] {
        &[Ability::Adult, Ability::Teen, Ability::Child]
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Ability::Adult => "Adult",
            Ability::Teen => "Teen",
            Ability::Child => "Child",
        }
    }

    pub fn from_i32(prim: i32) -> Self {
        match prim {
            0 => Ability::Adult,
            1 => Ability::Teen,
            2 => Ability::Child,
            _ => panic!("bad value for Ability: {}", prim),
        }
    }
}

impl From<Ability> for i32 {
    fn from(ability: Ability) -> i32 {
        ability as i32
    }
}

impl Display for Ability {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", self.to_str())
    }
}
