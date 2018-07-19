extern crate kp_chart;
#[cfg(feature = "web-spa")]
#[macro_use]
extern crate yew;

use std::iter::*;

use kp_chart::*;

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

    let context = Context {
        console: ConsoleService::new(),
    };

    let days = yew::initialize();

    let app: App<_, RootModel> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
