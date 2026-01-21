use dioxus::prelude::*;
use crate::components::*;
use lucide_rust::dioxus::*;

#[derive(Clone, Debug)]
struct EventData {
    id: String,
    title: String,
    description: String,
    date: String,
    location: String,
    tags: Vec<String>,
    image: Option<String>,
}

#[derive(Clone, Debug)]
struct EventRegistration {
    id: String,
    event_id: String,
    status: String,
    registration_date: String,
}

#[component]
pub fn RegisteredEventsPage() -> Element {
    let auth = use_context::<crate::contexts::AuthContext>();
    let mut search_query = use_signal(String::new);
    let mut filter_status = use_signal(|| String::from("all"));
    let mut active_tab = use_signal(|| String::from("upcoming"));

    // Mock event data
    let mock_events = vec![
        EventData {
            id: "1".to_string(),
            title: "Tech Summit 2026".to_string(),
            description: "Annual technology summit bringing together industry leaders".to_string(),
            date: "2026-03-15".to_string(),
            location: "San Francisco Convention Center".to_string(),
            tags: vec!["technology".to_string(), "conference".to_string(), "networking".to_string()],
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=500".to_string()),
        },
        EventData {
            id: "2".to_string(),
            title: "Networking Night".to_string(),
            description: "Casual networking event for professionals in tech".to_string(),
            date: "2026-02-28".to_string(),
            location: "Downtown Tech Hub".to_string(),
            tags: vec!["networking".to_string(), "social".to_string()],
            image: None,
        },
        EventData {
            id: "3".to_string(),
            title: "Spring Workshop".to_string(),
            description: "Intensive hands-on workshop for advanced skills".to_string(),
            date: "2026-04-10".to_string(),
            location: "Innovation Center".to_string(),
            tags: vec!["workshop".to_string(), "training".to_string()],
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=500".to_string()),
        },
        EventData {
            id: "4".to_string(),
            title: "Annual Gala".to_string(),
            description: "Formal networking event celebrating community achievements".to_string(),
            date: "2025-12-15".to_string(),
            location: "Grand Hotel Ballroom".to_string(),
            tags: vec!["formal".to_string(), "celebration".to_string()],
            image: None,
        },
        EventData {
            id: "5".to_string(),
            title: "Community Meetup".to_string(),
            description: "Monthly casual meetup for community members".to_string(),
            date: "2025-11-20".to_string(),
            location: "Local Coffee Shop".to_string(),
            tags: vec!["meetup".to_string(), "community".to_string()],
            image: None,
        },
    ];

    // Mock registrations
    let registrations = vec![
        EventRegistration {
            id: "r1".to_string(),
            event_id: "1".to_string(),
            status: "registered".to_string(),
            registration_date: "2026-01-10".to_string(),
        },
        EventRegistration {
            id: "r2".to_string(),
            event_id: "2".to_string(),
            status: "registered".to_string(),
            registration_date: "2026-01-15".to_string(),
        },
        EventRegistration {
            id: "r3".to_string(),
            event_id: "3".to_string(),
            status: "registered".to_string(),
            registration_date: "2026-01-12".to_string(),
        },
        EventRegistration {
            id: "r4".to_string(),
            event_id: "4".to_string(),
            status: "checked-in".to_string(),
            registration_date: "2025-10-01".to_string(),
        },
        EventRegistration {
            id: "r5".to_string(),
            event_id: "5".to_string(),
            status: "checked-in".to_string(),
            registration_date: "2025-10-15".to_string(),
        },
    ];

    // Filter registrations with events
    let user_registrations: Vec<(EventRegistration, EventData)> = registrations
        .iter()
        .filter_map(|reg| {
            mock_events
                .iter()
                .find(|e| e.id == reg.event_id)
                .map(|event| (reg.clone(), event.clone()))
        })
        .collect();

    // Apply search and filter
    let filtered_registrations: Vec<(EventRegistration, EventData)> = user_registrations
        .iter()
        .filter(|(reg, event)| {
            let search_match = event.title.to_lowercase().contains(&search_query.read().to_lowercase())
                || event.location.to_lowercase().contains(&search_query.read().to_lowercase());
            
            let filter_match = filter_status.read().as_str() == "all" || reg.status == filter_status.read().as_str();
            
            search_match && filter_match
        })
        .cloned()
        .collect();

    // Separate upcoming and past
    let now = chrono::Local::now();
    let upcoming: Vec<_> = filtered_registrations
        .iter()
        .filter(|(_, event)| {
            if let Ok(event_date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", event.date)) {
                event_date.timestamp() > now.timestamp()
            } else {
                false
            }
        })
        .cloned()
        .collect();

    let past: Vec<_> = filtered_registrations
        .iter()
        .filter(|(_, event)| {
            if let Ok(event_date) = chrono::DateTime::parse_from_rfc3339(&format!("{}T00:00:00Z", event.date)) {
                event_date.timestamp() <= now.timestamp()
            } else {
                false
            }
        })
        .cloned()
        .collect();

    let attended_count = user_registrations
        .iter()
        .filter(|(reg, _)| reg.status == "checked-in")
        .count();

    let cancelled_count = user_registrations
        .iter()
        .filter(|(reg, _)| reg.status == "cancelled")
        .count();

    rsx! {
        div {
            class: "container mx-auto py-8 px-4 md:px-6 max-w-7xl",
            PageHeader {
                title: "My Events",
                description: "Events you've registered for and attended",
            }

            // Search and Filters
            div {
                class: "flex flex-col sm:flex-row gap-4 mb-6",
                div {
                    class: "relative flex-1",
                    Search {
                        class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4",
                    }
                    Input {
                        placeholder: "Search your events...",
                        value: search_query.read().clone(),
                        onchange: move |val: String| {
                            search_query.set(val);
                        },
                        class: "pl-10",
                    }
                }

                Select {
                    value: filter_status.read().clone(),
                    onchange: move |val: String| {
                        filter_status.set(val);
                    },
                    class: "w-full sm:w-48",
                    {rsx! {
                        option { value: "all", "All Events" }
                        option { value: "registered", "Registered" }
                        option { value: "checked-in", "Attended" }
                        option { value: "cancelled", "Cancelled" }
                    }}
                }
            }

            // Stats Summary
            div {
                class: "grid grid-cols-1 sm:grid-cols-4 gap-4 mb-8",
                StatsCard {
                    icon: rsx! { Calendar { class: "h-5 w-5 text-blue-600" } },
                    value: user_registrations.len().to_string(),
                    label: "Total Events",
                    bg_color: "bg-blue-100",
                }
                StatsCard {
                    icon: rsx! { CheckCircle { class: "h-5 w-5 text-green-600" } },
                    value: upcoming.len().to_string(),
                    label: "Upcoming",
                    bg_color: "bg-green-100",
                }
                StatsCard {
                    icon: rsx! { Eye { class: "h-5 w-5 text-purple-600" } },
                    value: attended_count.to_string(),
                    label: "Attended",
                    bg_color: "bg-purple-100",
                }
                StatsCard {
                    icon: rsx! { XCircle { class: "h-5 w-5 text-orange-600" } },
                    value: cancelled_count.to_string(),
                    label: "Cancelled",
                    bg_color: "bg-orange-100",
                }
            }

            // Tabs
            div {
                class: "w-full",
                div {
                    class: "grid w-full grid-cols-2 gap-4 mb-6 border-b",
                    button {
                        class: if active_tab.read().as_str() == "upcoming" {
                            "pb-3 border-b-2 border-blue-600 text-blue-600 font-semibold"
                        } else {
                            "pb-3 text-gray-600 hover:text-gray-900"
                        },
                        onclick: move |_| {
                            active_tab.set("upcoming".to_string());
                        },
                        "Upcoming Events ({upcoming.len()})"
                    }
                    button {
                        class: if active_tab.read().as_str() == "past" {
                            "pb-3 border-b-2 border-blue-600 text-blue-600 font-semibold"
                        } else {
                            "pb-3 text-gray-600 hover:text-gray-900"
                        },
                        onclick: move |_| {
                            active_tab.set("past".to_string());
                        },
                        "Past Events ({past.len()})"
                    }
                }

                // Upcoming Tab
                if active_tab.read().as_str() == "upcoming" {
                    if upcoming.is_empty() {
                        Card {
                            class: "text-center py-12",
                            div {
                                class: "text-gray-400 mb-4",
                                Calendar { class: "h-12 w-12 mx-auto" }
                            }
                            h3 { class: "text-lg font-semibold text-gray-900 mb-2", "No upcoming events" }
                            p { class: "text-gray-600 mb-4", "You haven't registered for any upcoming events yet" }
                            Button {
                                variant: crate::components::button::ButtonVariant::Default,
                                "Browse Events"
                            }
                        }
                    } else {
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-6",
                            for (reg, event) in upcoming.iter() {
                                EventCard {
                                    registration: reg.clone(),
                                    event: event.clone(),
                                }
                            }
                        }
                    }
                }

                // Past Tab
                if active_tab.read().as_str() == "past" {
                    if past.is_empty() {
                        Card {
                            class: "text-center py-12 mt-6",
                            div {
                                class: "text-gray-400 mb-4",
                                Clock { class: "h-12 w-12 mx-auto" }
                            }
                            h3 { class: "text-lg font-semibold text-gray-900 mb-2", "No past events" }
                            p { class: "text-gray-600", "You haven't attended any past events yet" }
                        }
                    } else {
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-6",
                            for (reg, event) in past.iter() {
                                EventCard {
                                    registration: reg.clone(),
                                    event: event.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatsCard(icon: Element, value: String, label: String, bg_color: String) -> Element {
    rsx! {
        Card {
            div {
                class: "flex items-center p-4",
                div {
                    class: "p-2 {bg_color} rounded-lg mr-3",
                    {icon}
                }
                div {
                    p { class: "text-xl font-bold text-black", "{value}" }
                    p { class: "text-xs text-gray-600", "{label}" }
                }
            }
        }
    }
}

#[component]
fn EventCard(registration: EventRegistration, event: EventData) -> Element {
    let status_badge = match registration.status.as_str() {
        "checked-in" => Badge {
            variant: crate::components::badge::BadgeVariant::Outline,
            class: "border-green-200 text-green-700",
            "Attended"
        },
        "cancelled" => Badge {
            variant: crate::components::badge::BadgeVariant::Outline,
            class: "border-red-200 text-red-700",
            "Cancelled"
        },
        _ => Badge {
            variant: crate::components::badge::BadgeVariant::Outline,
            class: "border-blue-200 text-blue-700",
            "Registered"
        },
    };

    rsx! {
        Card {
            class: "overflow-hidden hover:shadow-md transition-shadow",
            if let Some(image) = &event.image {
                div {
                    class: "h-32 bg-cover bg-center",
                    style: "background-image: url('{image}')",
                }
            }
            div {
                class: "p-4",
                div {
                    class: "flex items-start justify-between mb-2",
                    div {
                        class: "flex-1",
                        h3 { class: "text-lg font-semibold line-clamp-2", "{event.title}" }
                        p { class: "text-sm text-gray-600 line-clamp-2 mt-1", "{event.description}" }
                    }
                    {status_badge}
                }
            }
            div {
                class: "px-4 pb-4 pt-0",
                div {
                    class: "space-y-2 mb-4",
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        Calendar { class: "mr-2 h-4 w-4 flex-shrink-0" }
                        span {
                            "{event.date}"
                        }
                    }
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        MapPin { class: "mr-2 h-4 w-4 flex-shrink-0" }
                        span { class: "line-clamp-1", "{event.location}" }
                    }
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        Clock { class: "mr-2 h-4 w-4 flex-shrink-0" }
                        span {
                            "Registered on {registration.registration_date}"
                        }
                    }
                }

                div {
                    class: "flex flex-wrap gap-1 mb-4",
                    for (i, tag) in event.tags.iter().enumerate() {
                        if i < 3 {
                            Badge {
                                variant: crate::components::badge::BadgeVariant::Outline,
                                class: "text-xs",
                                "{tag}"
                            }
                        }
                    }
                    if event.tags.len() > 3 {
                        Badge {
                            variant: crate::components::badge::BadgeVariant::Outline,
                            class: "text-xs",
                            "+{}"
                            (event.tags.len() - 3)
                        }
                    }
                }

                Button {
                    variant: crate::components::button::ButtonVariant::Default,
                    class: "w-full",
                    Eye { class: "mr-2 h-4 w-4" }
                    "View Event"
                }
            }
        }
    }
}
