use dioxus::prelude::*;

pub mod landing;

#[component]
pub fn CSS() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
    }
}
