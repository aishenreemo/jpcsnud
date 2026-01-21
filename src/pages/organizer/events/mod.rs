use dioxus::prelude::*;
use crate::contexts::use_auth;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::input::Input;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::page_header::PageHeader;
use crate::components::select::Select;
use crate::components::table::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};

pub mod new;

use lucide_rust::dioxus::{
    calendar_icon::Calendar,
    map_pin_icon::MapPin,
    users_icon::Users,
    search_icon::Search,
    filter_icon::Filter,
    plus_icon::Plus,
    edit_icon::Edit,
    eye_icon::Eye,
    more_horizontal_icon::MoreHorizontal,
    copy_icon::Copy,
    trash_2_icon::Trash2,
    user_check_icon::UserCheck,
};

#[derive(Clone, PartialEq, Debug)]
struct Event {
    id: String,
    title: String,
    description: String,
    date: String,
    location: String,
    status: String,
    tags: Vec<String>,
    max_attendees: Option<u32>,
}

#[component]
pub fn OrganizerEventsPage() -> Element {
    let auth = use_auth();
    let mut search_query = use_signal(|| String::new);
    let mut filter_status = use_signal(|| String::from("all"));
    let mut view_mode = use_signal(|| String::from("grid"));

    // Check if authenticated
    if (auth.user)().is_none() {
        return rsx! {
            div {
                class: "flex items-center justify-center min-h-screen",
                div {
                    class: "animate-spin rounded-full h-8 w-8 border-b-2 border-black"
                }
            }
        };
    }

    // Mock events data
    let all_events = vec![
        Event {
            id: "1".to_string(),
            title: "Tech Summit 2026".to_string(),
            description: "Annual technology conference".to_string(),
            date: "2026-02-15".to_string(),
            location: "San Francisco, CA".to_string(),
            status: "published".to_string(),
            tags: vec!["tech".to_string(), "conference".to_string(), "networking".to_string()],
            max_attendees: Some(500),
        },
        Event {
            id: "2".to_string(),
            title: "Networking Night".to_string(),
            description: "Casual networking event for members".to_string(),
            date: "2026-01-30".to_string(),
            location: "Los Angeles, CA".to_string(),
            status: "published".to_string(),
            tags: vec!["networking".to_string(), "social".to_string()],
            max_attendees: Some(100),
        },
        Event {
            id: "3".to_string(),
            title: "Workshop: Rust Basics".to_string(),
            description: "Learn the basics of Rust programming".to_string(),
            date: "2026-02-01".to_string(),
            location: "New York, NY".to_string(),
            status: "draft".to_string(),
            tags: vec!["workshop".to_string(), "rust".to_string(), "programming".to_string()],
            max_attendees: Some(50),
        },
        Event {
            id: "4".to_string(),
            title: "Annual Meetup".to_string(),
            description: "End of year celebration".to_string(),
            date: "2025-12-20".to_string(),
            location: "Chicago, IL".to_string(),
            status: "completed".to_string(),
            tags: vec!["celebration".to_string(), "meetup".to_string()],
            max_attendees: Some(200),
        },
    ];

    let total_events = all_events.len();
    let published_count = all_events.iter().filter(|e| e.status == "published").count();
    let draft_count = all_events.iter().filter(|e| e.status == "draft").count();
    let total_registrations = 245; // Mock count

    rsx! {
        div {
            class: "container mx-auto py-8 px-4 md:px-6 max-w-7xl",
            PageHeader {
                title: "Event Management",
                description: "Create, edit, and manage all your events"
            }

            // Search and Filters
            div {
                class: "flex flex-col sm:flex-row gap-4 mb-6",
                div {
                    class: "relative flex-1",
                    Search {
                        class: "absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4"
                    }
                    Input {
                        placeholder: "Search events...",
                        value: search_query(),
                        onchange: move |val: String| {
                            search_query.set(val);
                        },
                    }
                }
                
                Select {
                    options: vec![
                        ("all", "All Events"),
                        ("published", "Published"),
                        ("draft", "Draft"),
                        ("cancelled", "Cancelled"),
                        ("completed", "Completed"),
                    ],
                    value: filter_status(),
                    onchange: move |val: String| {
                        filter_status.set(val);
                    },
                }

                div {
                    class: "flex border rounded-md",
                    Button {
                        variant: if view_mode() == "grid" { ButtonVariant::Default } else { ButtonVariant::Ghost },
                        size: ButtonSize::Sm,
                        class: "rounded-r-none",
                        onclick: move |_| {
                            view_mode.set("grid".to_string());
                        },
                        "Grid"
                    }
                    Button {
                        variant: if view_mode() == "table" { ButtonVariant::Default } else { ButtonVariant::Ghost },
                        size: ButtonSize::Sm,
                        class: "rounded-l-none",
                        onclick: move |_| {
                            view_mode.set("table".to_string());
                        },
                        "Table"
                    }
                }
            }

            // Stats Summary
            div {
                class: "grid grid-cols-1 sm:grid-cols-4 gap-4 mb-8",
                EventStatsCard {
                    icon_bg: "bg-blue-100",
                    icon_color: "text-blue-600",
                    count: total_events.to_string(),
                    label: "Total Events",
                    icon: rsx! {
                        Calendar {
                            class: "h-5 w-5 text-blue-600"
                        }
                    }
                }
                EventStatsCard {
                    icon_bg: "bg-green-100",
                    icon_color: "text-green-600",
                    count: published_count.to_string(),
                    label: "Published",
                    icon: rsx! {
                        Eye {
                            class: "h-5 w-5 text-green-600"
                        }
                    }
                }
                EventStatsCard {
                    icon_bg: "bg-yellow-100",
                    icon_color: "text-yellow-600",
                    count: draft_count.to_string(),
                    label: "Drafts",
                    icon: rsx! {
                        Edit {
                            class: "h-5 w-5 text-yellow-600"
                        }
                    }
                }
                EventStatsCard {
                    icon_bg: "bg-purple-100",
                    icon_color: "text-purple-600",
                    count: total_registrations.to_string(),
                    label: "Total Registrations",
                    icon: rsx! {
                        Users {
                            class: "h-5 w-5 text-purple-600"
                        }
                    }
                }
            }

            // Events Display
            if all_events.is_empty() {
                rsx! {
                    Card {
                        class: "text-center py-12",
                        CardContent {
                            div {
                                class: "text-gray-400 mb-4",
                                Calendar {
                                    class: "h-12 w-12 mx-auto"
                                }
                            }
                            h3 {
                                class: "text-lg font-semibold text-gray-900 mb-2",
                                "No events found"
                            }
                            p {
                                class: "text-gray-600",
                                "You haven't created any events yet"
                            }
                        }
                    }
                }
            } else if view_mode() == "grid" {
                rsx! {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6",
                        {all_events.iter().map(|event| {
                            let event_clone = event.clone();
                            rsx! {
                                EventCard {
                                    key: "{event.id}",
                                    event: event_clone
                                }
                            }
                        }).collect::<Vec<_>>()}
                    }
                }
            } else {
                rsx! {
                    Card {
                        CardContent {
                            class: "p-0",
                            Table {
                                TableHeader {
                                    TableRow {
                                        TableHead { "Event" }
                                        TableHead { "Date" }
                                        TableHead { "Status" }
                                        TableHead { "Registrations" }
                                        TableHead { "Location" }
                                        TableHead {
                                            class: "text-right",
                                            "Actions"
                                        }
                                    }
                                }
                                TableBody {
                                    {all_events.iter().map(|event| {
                                        let event_clone = event.clone();
                                        rsx! {
                                            TableRow {
                                                key: "{event.id}",
                                                TableCell {
                                                    div {
                                                        p {
                                                            class: "font-medium line-clamp-1",
                                                            "{event.title}"
                                                        }
                                                        p {
                                                            class: "text-sm text-gray-500 line-clamp-1",
                                                            "{event.description}"
                                                        }
                                                    }
                                                }
                                                TableCell {
                                                    "{event.date}"
                                                }
                                                TableCell {
                                                    {get_status_badge(&event.status)}
                                                }
                                                TableCell {
                                                    "15"
                                                    if let Some(max) = event.max_attendees {
                                                        "/{max}"
                                                    }
                                                }
                                                TableCell {
                                                    span {
                                                        class: "max-w-[200px] truncate",
                                                        "{event.location}"
                                                    }
                                                }
                                                TableCell {
                                                    class: "text-right",
                                                    Button {
                                                        variant: ButtonVariant::Ghost,
                                                        size: ButtonSize::Sm,
                                                        MoreHorizontal {
                                                            class: "h-4 w-4"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }).collect::<Vec<_>>()}
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
fn EventStatsCard(icon_bg: String, icon_color: String, count: String, label: String, icon: Element) -> Element {
    rsx! {
        Card {
            CardContent {
                class: "flex items-center p-4",
                div {
                    class: format!("p-2 rounded-lg mr-3 {}", icon_bg),
                    {icon}
                }
                div {
                    p {
                        class: "text-xl font-bold text-black",
                        "{count}"
                    }
                    p {
                        class: "text-xs text-gray-600",
                        "{label}"
                    }
                }
            }
        }
    }
}

#[component]
fn EventCard(event: Event) -> Element {
    rsx! {
        Card {
            class: "overflow-hidden hover:shadow-lg transition-shadow",
            CardHeader {
                class: "pb-3",
                div {
                    class: "flex items-start justify-between mb-2",
                    div {
                        class: "flex-1",
                        CardTitle {
                            class: "text-lg line-clamp-2",
                            "{event.title}"
                        }
                        CardDescription {
                            class: "line-clamp-2 mt-1",
                            "{event.description}"
                        }
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        class: "h-8 w-8 p-0",
                        MoreHorizontal {
                            class: "h-4 w-4"
                        }
                    }
                }
                {get_status_badge(&event.status)}
            }
            CardContent {
                class: "pt-0",
                div {
                    class: "space-y-2 mb-4",
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        Calendar {
                            class: "mr-2 h-4 w-4 flex-shrink-0"
                        }
                        span {
                            "{event.date}"
                        }
                    }
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        MapPin {
                            class: "mr-2 h-4 w-4 flex-shrink-0"
                        }
                        span {
                            class: "line-clamp-1",
                            "{event.location}"
                        }
                    }
                    div {
                        class: "flex items-center text-sm text-gray-600",
                        Users {
                            class: "mr-2 h-4 w-4 flex-shrink-0"
                        }
                        span {
                            "15 registered"
                            if let Some(max) = event.max_attendees {
                                " of {max}"
                            }
                        }
                    }
                }
                
                div {
                    class: "flex flex-wrap gap-1 mb-4",
                    {event.tags.iter().take(3).map(|tag| {
                        rsx! {
                            Badge {
                                key: "{tag}",
                                variant: BadgeVariant::Outline,
                                class: "text-xs",
                                "{tag}"
                            }
                        }
                    }).collect::<Vec<_>>()}
                    if event.tags.len() > 3 {
                        Badge {
                            variant: BadgeVariant::Outline,
                            class: "text-xs",
                            "+{}"(event.tags.len() - 3)
                        }
                    }
                }

                div {
                    class: "flex space-x-2",
                    Button {
                        variant: ButtonVariant::Default,
                        size: ButtonSize::Sm,
                        class: "flex-1",
                        Edit {
                            class: "mr-2 h-4 w-4"
                        }
                        "Edit"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        UserCheck {
                            class: "h-4 w-4"
                        }
                    }
                }
            }
        }
    }
}

fn get_status_badge(status: &str) -> Element {
    match status {
        "published" => rsx! {
            Badge {
                variant: BadgeVariant::Outline,
                class: "border-green-200 text-green-700",
                "Published"
            }
        },
        "draft" => rsx! {
            Badge {
                variant: BadgeVariant::Outline,
                class: "border-yellow-200 text-yellow-700",
                "Draft"
            }
        },
        "cancelled" => rsx! {
            Badge {
                variant: BadgeVariant::Outline,
                class: "border-red-200 text-red-700",
                "Cancelled"
            }
        },
        "completed" => rsx! {
            Badge {
                variant: BadgeVariant::Outline,
                class: "border-blue-200 text-blue-700",
                "Completed"
            }
        },
        _ => rsx! {
            Badge {
                variant: BadgeVariant::Outline,
                "{status}"
            }
        },
    }
}