use dioxus::prelude::*;
use crate::components::Login;

#[component]
pub fn LogIn() -> Element {
    rsx! {
        h1 {"hello"}
        Login {}
    }
}
