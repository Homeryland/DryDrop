use dioxus::prelude::*;
use dioxus_components::landing::navbar::Navbar;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-background text-foreground",
            Navbar {}
        }
    }
}
