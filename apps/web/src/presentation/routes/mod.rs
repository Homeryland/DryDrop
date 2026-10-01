use crate::presentation::routes::home::Home;
use dioxus::prelude::*;

pub mod home;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home
}
