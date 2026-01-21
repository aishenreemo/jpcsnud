use dioxus::prelude::*;
use crate::components::*;
use lucide_rust::dioxus::*;

#[derive(Clone, Debug)]
struct Event {
    id: String,
    title: String,
    description: String,
    date: String,
    end_date: Option<String>,
    location: String,
    is_public: bool,
    price: Option<u32>,
    status: String,
    image: Option<String>,
    current_attendees: u32,
    max_attendees: Option<u32>,
    tags: Vec<String>,
    slug: String,
    requires_registration: bool,
    registration_deadline: Option<String>,
    organizer_name: String,
    organizer_email: String,
}

#[derive(Clone, Debug)]
struct EventRegistration {
    id: String,
    event_id: String,
    registration_date: String,
    status: String,
    notes: Option<String>,
}

#[component]
pub fn EventDetailPage() -> Element {
    let auth = use_context::<crate::contexts::AuthContext>();
    let mut is_registering = use_signal(|| false);
    let mut registration_notes = use_signal(String::new);

    // Mock events data
    let mock_events = vec![
        Event {
            id: "1".to_string(),
            title: "Tech Summit 2026".to_string(),
            description: "Annual technology summit bringing together industry leaders and innovators".to_string(),
            date: "2026-03-15".to_string(),
            end_date: Some("2026-03-17".to_string()),
            location: "San Francisco Convention Center".to_string(),
            is_public: true,
            price: Some(99),
            status: "published".to_string(),
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=800".to_string()),
            current_attendees: 245,
            max_attendees: Some(500),
            tags: vec!["technology".to_string(), "conference".to_string(), "networking".to_string()],
            slug: "tech-summit-2026".to_string(),
            requires_registration: true,
            registration_deadline: Some("2026-03-10".to_string()),
            organizer_name: "Tech Events Inc".to_string(),
            organizer_email: "info@techevents.com".to_string(),
        },
        Event {
            id: "2".to_string(),
            title: "Networking Night".to_string(),
            description: "Casual networking event for professionals in tech and startups".to_string(),
            date: "2026-02-28".to_string(),
            end_date: None,
            location: "Downtown Tech Hub".to_string(),
            is_public: true,
            price: None,
            status: "published".to_string(),
            image: None,
            current_attendees: 67,
            max_attendees: Some(150),
            tags: vec!["networking".to_string(), "social".to_string()],
            slug: "networking-night".to_string(),
            requires_registration: false,
            registration_deadline: None,
            organizer_name: "Community Team".to_string(),
            organizer_email: "community@example.com".to_string(),
        },
        Event {
            id: "3".to_string(),
            title: "Advanced Rust Workshop".to_string(),
            description: "Intensive hands-on workshop for advanced Rust programming techniques".to_string(),
            date: "2026-04-10".to_string(),
            end_date: None,
            location: "Innovation Center".to_string(),
            is_public: false,
            price: Some(149),
            status: "published".to_string(),
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=800".to_string()),
            current_attendees: 32,
            max_attendees: Some(50),
            tags: vec!["workshop".to_string(), "training".to_string(), "programming".to_string()],
            slug: "advanced-rust-workshop".to_string(),
            requires_registration: true,
            registration_deadline: Some("2026-04-05".to_string()),
            organizer_name: "Learning Academy".to_string(),
            organizer_email: "academy@example.com".to_string(),
        },
    ];

    // Mock registrations
    let mock_registrations = vec![
        EventRegistration {
            id: "r1".to_string(),
            event_id: "1".to_string(),
            registration_date: "2026-01-10".to_string(),
            status: "registered".to_string(),
            notes: Some("I have accessibility needs".to_string()),
        },
    ];

    // Get event from slug (would be from route params in real app)
    let event = mock_events.first().cloned();

    match event {
        None => {
            rsx! {
                div {
                    class: "container mx-auto py-16 px-4 md:px-6 text-center max-w-2xl",
                    h1 { class: "text-2xl font-bold mb-4", "Event Not Found" }
                    p { class: "text-gray-600 mb-6", "The event you're looking for doesn't exist or has been removed." }
                    Button {
                        variant: crate::components::button::ButtonVariant::Default,
                        ArrowLeft { class: "mr-2 h-4 w-4" }
                        "Back to Events"
                    }
                }
            }
        }
        Some(event) => {
            let is_authenticated = (auth)().is_some();
            let can_view = event.is_public || is_authenticated;

            if !can_view {
                return rsx! {
                    div {
                        class: "container mx-auto py-16 px-4 md:px-6 text-center max-w-2xl",
                        div { class: "mb-6",
                            Lock { class: "h-16 w-16 mx-auto text-gray-400 mb-4" }
                        }
                        h1 { class: "text-2xl font-bold mb-4", "Members Only Event" }
                        p { class: "text-gray-600 mb-6", "This event is restricted to members only. Please sign in or create an account to view details." }
                        div {
                            class: "flex flex-col sm:flex-row gap-4 justify-center",
                            Button {
                                variant: crate::components::button::ButtonVariant::Default,
                                "Join Now"
                            }
                            Button {
                                variant: crate::components::button::ButtonVariant::Outline,
                                "Sign In"
                            }
                        }
                    }
                };
            }

            let existing_registration = mock_registrations.iter().find(|r| r.event_id == event.id);
            let is_event_past = event.date.as_str() < "2026-01-21";
            let is_registration_open = event.requires_registration
                && event.registration_deadline.as_ref().map_or(true, |d| d.as_str() > "2026-01-21")
                && !is_event_past;

            let attendee_percentage = if let Some(max) = event.max_attendees {
                ((event.current_attendees as f32 / max as f32) * 100.0).min(100.0) as u32
            } else {
                0
            };

            let organizer_initials = event.organizer_name
                .split_whitespace()
                .map(|w| w.chars().next().unwrap_or('?'))
                .collect::<String>()
                .to_uppercase();

            rsx! {
                div {
                    class: "container mx-auto py-8 px-4 md:px-6 max-w-4xl",

                    // Back Button
                    div {
                        class: "mb-6",
                        Button {
                            variant: crate::components::button::ButtonVariant::Ghost,
                            ArrowLeft { class: "mr-2 h-4 w-4" }
                            "Back to Events"
                        }
                    }

                    // Event Header
                    div {
                        class: "mb-8",
                        if let Some(image) = &event.image {
                            div {
                                class: "h-64 md:h-80 rounded-lg bg-cover bg-center mb-6",
                                style: "background-image: url('{image}')",
                            }
                        }

                        div {
                            class: "flex flex-col md:flex-row md:items-start md:justify-between gap-4 mb-4",
                            div {
                                class: "flex-1",
                                div {
                                    class: "flex flex-wrap items-center gap-2 mb-3",
                                    Badge {
                                        variant: if event.price.is_some() && event.price.unwrap_or(0) > 0 {
                                            crate::components::badge::BadgeVariant::Default
                                        } else {
                                            crate::components::badge::BadgeVariant::Secondary
                                        },
                                        if let Some(price) = event.price {
                                            if price > 0 {
                                                "{price}"
                                            } else {
                                                "Free"
                                            }
                                        } else {
                                            "Free"
                                        }
                                    }
                                    if !event.is_public {
                                        Badge {
                                            variant: crate::components::badge::BadgeVariant::Outline,
                                            Lock { class: "h-3 w-3 mr-1" }
                                            "Members Only"
                                        }
                                    }
                                    Badge {
                                        variant: crate::components::badge::BadgeVariant::Outline,
                                        class: match event.status.as_str() {
                                            "published" => "border-green-200 text-green-700",
                                            "draft" => "border-yellow-200 text-yellow-700",
                                            _ => "border-gray-200 text-gray-700",
                                        },
                                        "{event.status}"
                                    }
                                    for tag in event.tags.iter().take(3) {
                                        Badge {
                                            variant: crate::components::badge::BadgeVariant::Outline,
                                            class: "text-xs",
                                            "{tag}"
                                        }
                                    }
                                }
                                h1 { class: "text-3xl md:text-4xl font-bold text-black mb-2", "{event.title}" }
                                p { class: "text-lg text-gray-600", "{event.description}" }
                            }

                            div { class: "flex gap-2",
                                Button {
                                    variant: crate::components::button::ButtonVariant::Outline,
                                    Share2 { class: "h-4 w-4 mr-2" }
                                    "Share"
                                }
                            }
                        }
                    }

                    div {
                        class: "grid grid-cols-1 lg:grid-cols-3 gap-8",

                        // Main Content
                        div {
                            class: "lg:col-span-2",

                            // Event Details Card
                            Card {
                                class: "mb-6",
                                div { class: "p-6",
                                    h3 { class: "text-lg font-semibold mb-4", "Event Details" }
                                    div { class: "space-y-4",
                                        div { class: "flex items-center text-gray-700",
                                            Calendar { class: "mr-3 h-5 w-5 text-blue-500" }
                                            div {
                                                div { class: "font-medium", "{event.date}" }
                                                if let Some(end_date) = &event.end_date {
                                                    div { class: "text-sm text-gray-500", "Until {end_date}" }
                                                }
                                            }
                                        }

                                        div { class: "flex items-center text-gray-700",
                                            MapPin { class: "mr-3 h-5 w-5 text-red-500" }
                                            div {
                                                div { class: "font-medium", "{event.location}" }
                                                a {
                                                    href: "https://maps.google.com",
                                                    target: "_blank",
                                                    class: "text-sm text-blue-600 hover:underline flex items-center",
                                                    "View on Map"
                                                    ExternalLink { class: "ml-1 h-3 w-3" }
                                                }
                                            }
                                        }

                                        div { class: "flex items-center text-gray-700",
                                            Users { class: "mr-3 h-5 w-5 text-green-500" }
                                            div {
                                                div { class: "font-medium",
                                                    "{event.current_attendees} attendees"
                                                    if let Some(max) = event.max_attendees {
                                                        " of {max} max"
                                                    }
                                                }
                                                if event.max_attendees.is_some() {
                                                    div {
                                                        class: "w-full bg-gray-200 rounded-full h-2 mt-1",
                                                        div {
                                                            class: "bg-green-500 h-2 rounded-full",
                                                            style: "width: {attendee_percentage}%",
                                                        }
                                                    }
                                                }
                                            }
                                        }

                                        if let Some(price) = event.price {
                                            if price > 0 {
                                                div { class: "flex items-center text-gray-700",
                                                    DollarSign { class: "mr-3 h-5 w-5 text-yellow-500" }
                                                    div {
                                                        div { class: "font-medium", "${price}" }
                                                        div { class: "text-sm text-gray-500", "Per person" }
                                                    }
                                                }
                                            }
                                        }

                                        if let Some(deadline) = &event.registration_deadline {
                                            div { class: "flex items-center text-gray-700",
                                                Clock { class: "mr-3 h-5 w-5 text-purple-500" }
                                                div {
                                                    div { class: "font-medium", "Registration deadline" }
                                                    div { class: "text-sm text-gray-500", "{deadline}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Organizer Card
                            Card {
                                div { class: "p-6",
                                    h3 { class: "text-lg font-semibold mb-4", "Organizer" }
                                    div { class: "flex items-center space-x-3",
                                        div {
                                            class: "h-10 w-10 rounded-full bg-gray-200 flex items-center justify-center",
                                            span { class: "text-sm font-medium", "{organizer_initials}" }
                                        }
                                        div {
                                            div { class: "font-medium", "{event.organizer_name}" }
                                            div { class: "text-sm text-gray-500", "{event.organizer_email}" }
                                        }
                                    }
                                }
                            }
                        }

                        // Registration Sidebar
                        div {
                            class: "lg:col-span-1",
                            Card {
                                class: "sticky top-8",
                                div { class: "p-6",
                                    h3 { class: "text-lg font-semibold", 
                                        if existing_registration.is_some() {
                                            "Registration Confirmed"
                                        } else {
                                            "Event Registration"
                                        }
                                    }
                                    if let Some(_) = existing_registration {
                                        p { class: "text-sm text-green-600 font-medium mt-1", "You're registered for this event!" }
                                    } else {
                                        p { class: "text-sm text-gray-600 mt-1",
                                            if event.requires_registration {
                                                "Registration required to attend"
                                            } else {
                                                "Open event - no registration needed"
                                            }
                                        }
                                    }
                                }
                                div { class: "px-6 pb-6 pt-4",
                                    if existing_registration.is_some() {
                                        div { class: "space-y-3",
                                            div { class: "p-3 bg-green-50 border border-green-200 rounded-lg",
                                                div { class: "flex items-center space-x-2",
                                                    UserPlus { class: "h-5 w-5 text-green-600" }
                                                    span { class: "font-medium text-green-800", "Registered" }
                                                }
                                                p { class: "text-sm text-green-700 mt-1", "Registered on {existing_registration.unwrap().registration_date}" }
                                            }
                                            Button {
                                                variant: crate::components::button::ButtonVariant::Outline,
                                                class: "w-full",
                                                "View My Events"
                                            }
                                        }
                                    } else {
                                        if !is_authenticated {
                                            div { class: "space-y-3",
                                                p { class: "text-sm text-gray-600", "Sign in or create an account to register for this event." }
                                                Button {
                                                    variant: crate::components::button::ButtonVariant::Default,
                                                    class: "w-full",
                                                    "Sign In"
                                                }
                                                Button {
                                                    variant: crate::components::button::ButtonVariant::Outline,
                                                    class: "w-full",
                                                    "Create Account"
                                                }
                                            }
                                        } else if !event.requires_registration {
                                            div { class: "space-y-3",
                                                p { class: "text-sm text-gray-600", "This is an open event. No registration required - just show up!" }
                                                div { class: "p-3 bg-blue-50 border border-blue-200 rounded-lg",
                                                    p { class: "text-sm text-blue-800", "Don't forget to bring your RFID card for easy check-in!" }
                                                }
                                            }
                                        } else if !is_registration_open {
                                            div { class: "space-y-3",
                                                p { class: "text-sm text-red-600",
                                                    if is_event_past {
                                                        "This event has already passed"
                                                    } else if let Some(deadline) = &event.registration_deadline {
                                                        if deadline.as_str() < "2026-01-21" {
                                                            "Registration deadline has passed"
                                                        } else {
                                                            "Registration is currently closed"
                                                        }
                                                    } else if let Some(max) = event.max_attendees {
                                                        if event.current_attendees >= max {
                                                            "Event is fully booked"
                                                        } else {
                                                            "Registration is currently closed"
                                                        }
                                                    } else {
                                                        "Registration is currently closed"
                                                    }
                                                }
                                            }
                                        } else {
                                            div { class: "space-y-4",
                                                div { class: "space-y-2",
                                                    Label { for_id: "notes", "Notes (optional)" }
                                                    Textarea {
                                                        id: "notes",
                                                        placeholder: "Any special requirements or questions...",
                                                        value: registration_notes.read().clone(),
                                                        onchange: move |val: String| {
                                                            registration_notes.set(val);
                                                        },
                                                        rows: 3,
                                                    }
                                                }

                                                Button {
                                                    variant: crate::components::button::ButtonVariant::Default,
                                                    class: "w-full",
                                                    disabled: is_registering.read().clone(),
                                                    onclick: move |_| {
                                                        spawn({
                                                            let mut is_reg = is_registering.clone();
                                                            async move {
                                                                is_reg.set(true);
                                                                async_std::task::sleep(std::time::Duration::from_millis(1000)).await;
                                                                is_reg.set(false);
                                                            }
                                                        });
                                                    },
                                                    if is_registering.read().clone() {
                                                        "Registering..."
                                                    } else {
                                                        if let Some(price) = event.price {
                                                            if price > 0 {
                                                                "Register for Event - ${price}"
                                                            } else {
                                                                "Register for Event"
                                                            }
                                                        } else {
                                                            "Register for Event"
                                                        }
                                                    }
                                                }

                                                p { class: "text-xs text-gray-500", "By registering, you confirm your attendance for this event." }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
