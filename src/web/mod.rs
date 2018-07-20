use yew::services::console::ConsoleService;
use yew::services::storage::StorageService;

pub struct Context {
    pub console: ConsoleService,
    pub local_store: StorageService,
}

mod chart;
mod people;
mod root;

pub use self::chart::Chart;
pub use self::people::PeopleModel;
pub use self::root::RootModel;
