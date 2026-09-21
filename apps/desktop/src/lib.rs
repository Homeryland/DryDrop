use dioxus::prelude::*;
use dioxus_components::CSS;

use crate::presentation::routes::Route;

pub mod application;
pub mod presentation;

#[component]
pub fn App() -> Element {
    rsx! {
        CSS {}
        Router::<Route> {}
    }
}
