use dioxus::prelude::*;
use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::select::Select;
use lucide_rust::dioxus::calendar_icon::Calendar;
use lucide_rust::dioxus::search_icon::Search;
use crate::components::input::Input;
use crate::components::page_header::PageHeader;
use crate::contexts::use_auth;

pub mod event;
pub mod registered;

#[derive(Clone, Debug, PartialEq)]
struct Event {
    id: String,
    title: String,
    description: String,
    date: String,
    location: String,
    is_public: bool,
    price: Option<u32>,
    status: String,
    image: Option<String>,
    current_attendees: u32,
    max_attendees: Option<u32>,
    tags: Vec<String>,
    slug: String,
}

#[component]
pub fn EventsPage() -> Element {
    let auth = use_auth();
    let mut search_query = use_signal(String::new);
    let mut filter_status = use_signal(|| String::from("all"));
    let mut sort_by = use_signal(|| String::from("date"));

    // Mock events data
    let mock_events = vec![
        Event {
            id: "1".to_string(),
            title: "Tech Summit 2026".to_string(),
            description: "Annual technology summit bringing together industry leaders and innovators".to_string(),
            date: "2026-03-15".to_string(),
            location: "San Francisco Convention Center".to_string(),
            is_public: true,
            price: Some(99),
            status: "published".to_string(),
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=500".to_string()),
            current_attendees: 245,
            max_attendees: Some(500),
            tags: vec!["technology".to_string(), "conference".to_string(), "networking".to_string()],
            slug: "tech-summit-2026".to_string(),
        },
        Event {
            id: "2".to_string(),
            title: "Networking Night".to_string(),
            description: "Casual networking event for professionals in tech and startups".to_string(),
            date: "2026-02-28".to_string(),
            location: "Downtown Tech Hub".to_string(),
            is_public: true,
            price: None,
            status: "published".to_string(),
            image: None,
            current_attendees: 67,
            max_attendees: Some(150),
            tags: vec!["networking".to_string(), "social".to_string()],
            slug: "networking-night".to_string(),
        },
        Event {
            id: "3".to_string(),
            title: "Advanced Rust Workshop".to_string(),
            description: "Intensive hands-on workshop for advanced Rust programming techniques".to_string(),
            date: "2026-04-10".to_string(),
            location: "Innovation Center".to_string(),
            is_public: false,
            price: Some(149),
            status: "published".to_string(),
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=500".to_string()),
            current_attendees: 32,
            max_attendees: Some(50),
            tags: vec!["workshop".to_string(), "training".to_string(), "programming".to_string()],
            slug: "advanced-rust-workshop".to_string(),
        },
        Event {
            id: "4".to_string(),
            title: "Spring Community Meetup".to_string(),
            description: "Monthly casual meetup for community members to share ideas".to_string(),
            date: "2026-02-15".to_string(),
            location: "Local Coffee Shop".to_string(),
            is_public: true,
            price: None,
            status: "draft".to_string(),
            image: None,
            current_attendees: 24,
            max_attendees: Some(75),
            tags: vec!["meetup".to_string(), "community".to_string()],
            slug: "spring-meetup".to_string(),
        },
        Event {
            id: "5".to_string(),
            title: "Members Exclusive Gala".to_string(),
            description: "Formal networking event celebrating community achievements and milestones".to_string(),
            date: "2026-05-20".to_string(),
            location: "Grand Hotel Ballroom".to_string(),
            is_public: false,
            price: Some(250),
            status: "published".to_string(),
            image: Some("https://images.unsplash.com/photo-1552664730-d307ca884978?w=500".to_string()),
            current_attendees: 156,
            max_attendees: Some(300),
            tags: vec!["formal".to_string(), "celebration".to_string(), "members-only".to_string()],
            slug: "members-gala".to_string(),
        },
        Event {
            id: "6".to_string(),
            title: "Design Workshop Series".to_string(),
            description: "Multi-week workshop on modern UI/UX design principles".to_string(),
            date: "2026-03-01".to_string(),
            location: "Design Studio".to_string(),
            is_public: true,
            price: Some(199),
            status: "published".to_string(),
            image: None,
            current_attendees: 48,
            max_attendees: Some(100),
            tags: vec!["design".to_string(), "workshop".to_string(), "training".to_string()],
            slug: "design-workshop".to_string(),
        },
    ];

    // Filter events based on authentication
    let visible_events: Vec<Event> = mock_events
        .iter()
        .filter(|event| {
            if event.is_public {
                return true;
            }
            if !event.is_public && (auth.user)().is_some() {
                return true;
            }
            false
        })
        .cloned()
        .collect();

    // Apply search and filters
    let filtered_events: Vec<Event> = visible_events
        .iter()
        .filter(|event| {
            let search_match = event.title.to_lowercase().contains(&search_query.read().to_lowercase())
                || event.description.to_lowercase().contains(&search_query.read().to_lowercase())
                || event.location.to_lowercase().contains(&search_query.read().to_lowercase());

            let filter_match = match filter_status.read().as_str() {
                "all" => true,
                "public" => event.is_public,
                "private" => !event.is_public,
                "free" => event.price.is_none() || event.price == Some(0),
                "paid" => event.price.is_some() && event.price.unwrap_or(0) > 0,
                _ => true,
            };

            search_match && filter_match
        })
        .cloned()
        .collect();

    // Apply sorting
    let mut sorted_events = filtered_events.clone();
    match sort_by.read().as_str() {
        "date" => {
            sorted_events.sort_by(|a, b| a.date.cmp(&b.date));
        }
        "title" => {
            sorted_events.sort_by(|a, b| a.title.cmp(&b.title));
        }
        "attendees" => {
            sorted_events.sort_by(|a, b| b.current_attendees.cmp(&a.current_attendees));
        }
        _ => {}
    }

    let is_authenticated = (auth.user)().is_some();
    let description = if is_authenticated {
        "Discover all events and join our community activities"
    } else {
        "Discover public events and join our community activities"
    };

    rsx! {
        div {
            class: "container mx-auto py-6 sm:py-8 px-3 sm:px-4 md:px-6 max-w-7xl",
            PageHeader {
                title: "Events",
                description,
            }

            // Search and Filters
            div {
                class: "flex flex-col sm:flex-row gap-3 sm:gap-4 mb-6 sm:mb-8",
                div {
                    class: "relative flex-1",
                    Search {
                        class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4",
                    }
                    Input {
                        placeholder: "Search events...",
                        value: search_query.read().clone(),
                        class: "pl-10",
                    }
                }

                Select {
                    option { value: "all", "All Events" }
                    option { value: "public", "Public" }
                    if is_authenticated {
                        option { value: "private", "Members Only" }
                    }
                    option { value: "free", "Free" }
                    option { value: "paid", "Paid" }
                }

                Select {
                    option { value: "date", "Sort by Date" }
                    option { value: "title", "Sort by Title" }
                    option { value: "attendees", "Sort by Attendees" }
                }
            }

            // Results Summary
            div {
                class: "mb-6",
                p {
                    class: "text-gray-600",
                    "Showing {sorted_events.len()} of {visible_events.len()} events"
                    if !search_query.read().is_empty() {
                        " for \"{search_query}\""
                    }
                }
            }

            // Events Grid
            if sorted_events.is_empty() {
                Card {
                    class: "text-center py-12",
                    div {
                        class: "text-gray-400 mb-4",
                        Calendar { class: "h-12 w-12 mx-auto" }
                    }
                    h3 { class: "text-lg font-semibold text-gray-900 mb-2", "No events found" }
                    p {
                        class: "text-gray-600",
                        if !search_query.read().is_empty() {
                            "No events match your search for \"{search_query}\""
                        } else {
                            "No events are currently available"
                        }
                    }
                }
            } else {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6",
                    for event in sorted_events.iter() {
                        EventCard {
                            event: event.clone(),
                        }
                    }
                }
            }

            // Call to Action for Non-Authenticated Users
            if !is_authenticated {
                Card {
                    class: "mt-8 sm:mt-12 bg-gradient-to-r from-gray-50 to-blue-50 border-blue-200",
                    div {
                        class: "text-center py-6 sm:py-8 px-4",
                        h3 { class: "text-xl font-semibold mb-2", "Want to see member-only events?" }
                        p {
                            class: "text-gray-600 mb-6",
                            "Join our community to access exclusive workshops, networking events, and more!"
                        }
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
                }
            }
        }
    }
}

#[component]
fn EventCard(event: Event) -> Element {
    let status_color = match event.status.as_str() {
        "published" => "border-green-200 text-green-700",
        "draft" => "border-yellow-200 text-yellow-700",
        _ => "border-gray-200 text-gray-700",
    };

    let price_badge = if let Some(price) = event.price {
        if price > 0 {
            format!("${}", price)
        } else {
            "Free".to_string()
        }
    } else {
        "Free".to_string()
    };

    let price_variant = if event.price.is_some() && event.price.unwrap_or(0) > 0 {
        crate::components::badge::BadgeVariant::Default
    } else {
        crate::components::badge::BadgeVariant::Secondary
    };

    rsx! {
        Card {
            class: "overflow-hidden hover:shadow-lg transition-shadow",
            if let Some(image) = &event.image {
                div {
                    class: "h-48 bg-cover bg-center",
                    style: "background-image: url('{image}')",
                }
            }
            div {
                class: "p-4 sm:p-6",
                div {
                    class: "flex items-center justify-between mb-2",
                    div {
                        class: "flex items-center space-x-2",
                        Badge {
                            variant: price_variant,
                            "{price_badge}"
                        }
                        if !event.is_public {
                            Badge {
                                variant: crate::components::badge::BadgeVariant::Outline,
                                class: "flex items-center space-x-1",
                                Lock { class: "h-3 w-3" }
                                "Members"
                            }
                        }
                    }
                    Badge {
                        variant: crate::components::badge::BadgeVariant::Outline,
                        class: status_color,
                        "{event.status}"
                    }
                }
                h3 { class: "text-base sm:text-lg font-semibold line-clamp-2 mb-1", "{event.title}" }
                p { class: "text-sm text-gray-600 line-clamp-2", "{event.description}" }
            }
            div {
                class: "px-4 sm:px-6 pb-4 sm:pb-6 pt-0",
                div {
                    class: "space-y-2 mb-4",
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        Calendar { class: "mr-2 h-4 w-4 flex-shrink-0" }
                        span { class: "line-clamp-1", "{event.date}" }
                    }
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        MapPin { class: "mr-2 h-4 w-4 flex-shrink-0" }
                        span { class: "line-clamp-1", "{event.location}" }
                    }
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        Users { class: "mr-2 h-4 w-4 flex-shrink-0" }
                        span {
                            "{event.current_attendees} attendees"
                            if let Some(max) = event.max_attendees {
                                " of {max}"
                            }
                        }
                    }
                }

                div {
                    class: "flex flex-wrap gap-1 mb-3 sm:mb-4",
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
                    "View Details"
                }
            }
        }
    }
}
