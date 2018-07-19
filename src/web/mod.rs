use yew::prelude::*;
use yew::services::console::ConsoleService;

pub struct Context {
    pub console: ConsoleService,
}

mod chart;
mod people;
mod root;

pub use self::chart::Chart;
pub use self::people::People;
pub use self::root::RootModel;
