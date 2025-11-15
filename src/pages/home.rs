use dioxus::prelude::*;

use crate::contexts::use_auth;

#[component]
pub fn Home() -> Element {
    let auth = use_auth();

    rsx! {}
}
