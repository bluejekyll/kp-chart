extern crate kp_chart;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[cfg(feature = "web-spa")]
#[macro_use]
extern crate yew;
#[cfg(feature = "web-spa")]
extern crate stdweb;

#[cfg(not(feature = "web-spa"))]
fn main() {
    println!("Welcome to Kitchen Patrol Charts");

    calculate_day_jobs();
}

#[cfg(feature = "web-spa")]
mod web;

#[cfg(feature = "web-spa")]
fn main() {
    use web::*;
    use yew::prelude::*;
    use yew::services::console::ConsoleService;
    use yew::services::storage::{Area, StorageService};

    let context = Context {
        console: ConsoleService::new(),
        local_store: StorageService::new(Area::Local),
    };

    yew::initialize();

    let app: App<_, RootModel> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
