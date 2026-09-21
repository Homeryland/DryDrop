use dioxus::prelude::*;

use crate::landing::theme_toggle::ThemeToggle;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        section {
            class: "flex justify-between items-center px-4 py-3",
            div {
                class: "flex items-center gap-2.5",
                img {
                    src: asset!("/assets/img/logo.png"),
                    class: "h-10 w-auto"
                }
                h1 {
                    class: "text-2xl font-bold text-gray-900 dark:text-white",
                    "Homeryland"
                }
            }
            ThemeToggle {}
        }
    }
}
