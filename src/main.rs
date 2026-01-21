use dioxus::prelude::*;

use crate::components::Navbar;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::profile::Profile;
use crate::pages::dashboard::Dashboard;
use crate::pages::rfid::RfidRequest;
use crate::pages::events::Events;
use crate::pages::events::registered::RegisteredEvents;
use crate::pages::events::event::EventDetailPage;
use crate::pages::organizer::OrganizerDashboard;
use crate::pages::organizer::members::OrganizerMembers;
use crate::pages::organizer::events::OrganizerEvents;
use crate::pages::organizer::events::new::CreateEvent;
use crate::pages::organizer::rfid::RfidOrdersPage;

mod components;
mod contexts;
mod model;
mod pages;

#[derive(Debug, Clone, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/profile")]
    Profile {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/rfid")]
    RfidRequest {},
    #[route("/events")]
    Events {},
    #[route("/events/registered")]
    RegisteredEvents {},
    #[layout(Navbar)]
    #[route("/event")]
    EventDetailPage {},
    #[route("/organizer")]
    OrganizerDashboard {},
    #[route("/organizer/members")]
    OrganizerMembers {},
    #[route("/organizer/events")]
    OrganizerEvents {},
    #[route("/organizer/events/new")]
    CreateEvent {},
    #[route("/organizer/rfid")]
    RfidOrdersPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    contexts::use_auth_provider();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
