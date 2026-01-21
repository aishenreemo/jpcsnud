use dioxus::prelude::*;
use crate::contexts::use_auth;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::page_header::PageHeader;
use crate::Route;

use lucide_rust::dioxus::{
    calendar_icon::Calendar,
    credit_card_icon::CreditCard,
    users_icon::Users,
    clock_icon::Clock,
    check_circle_icon::CheckCircle,
    alert_circle_icon::AlertCircle,
    trending_up_icon::TrendingUp,
    map_pin_icon::MapPin,
    arrow_right_icon::ArrowRight,
};

#[component]
pub fn Dashboard() -> Element {
    let auth = use_auth();

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

    rsx! {
        div {
            class: "container mx-auto py-6 sm:py-8 px-3 sm:px-4 md:px-6 max-w-7xl",
            PageHeader {
                title: "Welcome back!",
                description: "Your membership dashboard and upcoming events"
            }

            // Stats Cards
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 mb-6 sm:mb-8",
                StatsCard {
                    icon_bg: "bg-blue-100",
                    icon_color: "text-blue-600",
                    title: "5",
                    subtitle: "Events Attended",
                    icon: rsx! {
                        Calendar {
                            class: "h-6 w-6 text-blue-600"
                        }
                    }
                }
                StatsCard {
                    icon_bg: "bg-green-100",
                    icon_color: "text-green-600",
                    title: "3",
                    subtitle: "Upcoming Events",
                    icon: rsx! {
                        Clock {
                            class: "h-6 w-6 text-green-600"
                        }
                    }
                }
                StatsCard {
                    icon_bg: "bg-purple-100",
                    icon_color: "text-purple-600",
                    title: "January 2026",
                    subtitle: "Member Since",
                    icon: rsx! {
                        Users {
                            class: "h-6 w-6 text-purple-600"
                        }
                    }
                }
                StatsCard {
                    icon_bg: "bg-orange-100",
                    icon_color: "text-orange-600",
                    title: "Active",
                    subtitle: "RFID Status",
                    icon: rsx! {
                        CreditCard {
                            class: "h-6 w-6 text-orange-600"
                        }
                    }
                }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6 sm:gap-8",
                // Left Column
                div {
                    class: "lg:col-span-2 space-y-6",
                    // Upcoming Events
                    UpcomingEventsCard {}
                    // Recent Activity
                    RecentActivityCard {}
                }

                // Right Column
                div {
                    class: "space-y-6",
                    ProfileCard {}
                    RfidCardStatusCard {}
                    QuickActionsCard {}
                }
            }
        }
    }
}

#[component]
fn StatsCard(icon_bg: String, icon_color: String, title: String, subtitle: String, icon: Element) -> Element {
    rsx! {
        Card {
            CardContent {
                class: "flex items-center p-4 sm:p-6",
                div {
                    class: format!("p-2 rounded-lg mr-4 {}", icon_bg),
                    {icon}
                }
                div {
                    p {
                        class: "text-2xl font-bold text-black",
                        "{title}"
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "{subtitle}"
                    }
                }
            }
        }
    }
}

#[component]
fn UpcomingEventsCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                class: "flex flex-row items-center justify-between",
                div {
                    CardTitle { "Upcoming Events" }
                    CardDescription { "Events you're registered for" }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        // Navigate to registered events
                    },
                    "View All "
                    ArrowRight {
                        class: "ml-2 h-4 w-4"
                    }
                }
            }
            CardContent {
                div {
                    class: "text-center py-8",
                    Calendar {
                        class: "h-12 w-12 text-gray-300 mx-auto mb-4"
                    }
                    p {
                        class: "text-gray-600 mb-4",
                        "No upcoming events registered"
                    }
                    Button {
                        onclick: move |_| {
                            // Navigate to events
                        },
                        "Browse Events"
                    }
                }
            }
        }
    }
}

#[component]
fn RecentActivityCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Recent Activity" }
                CardDescription { "Your latest event activity" }
            }
            CardContent {
                div {
                    class: "text-center py-4",
                    p {
                        class: "text-gray-600",
                        "No recent activity"
                    }
                }
            }
        }
    }
}

#[component]
fn ProfileCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Profile" }
            }
            CardContent {
                class: "space-y-4",
                div {
                    p {
                        class: "font-medium text-black",
                        "John Doe"
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "john@example.com"
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "ID: MEM-20260115-0001"
                    }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "w-full",
                    onclick: move |_| {
                        // Navigate to profile
                    },
                    "Edit Profile"
                }
            }
        }
    }
}

#[component]
fn RfidCardStatusCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "RFID Card" }
                CardDescription { "Your digital membership card" }
            }
            CardContent {
                class: "space-y-4",
                div {
                    class: "p-3 bg-green-50 border border-green-200 rounded-lg",
                    div {
                        class: "flex items-center space-x-2",
                        CheckCircle {
                            class: "h-5 w-5 text-green-600"
                        }
                        span {
                            class: "font-medium text-green-800",
                            "Card Active"
                        }
                    }
                    p {
                        class: "text-sm text-green-700 mt-1",
                        "Card #1234"
                    }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "w-full",
                    onclick: move |_| {
                        // Navigate to manage card
                    },
                    "Manage Card"
                }
            }
        }
    }
}

#[component]
fn QuickActionsCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Quick Actions" }
            }
            CardContent {
                class: "space-y-2",
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "w-full justify-start text-sm",
                    onclick: move |_| {
                        // Navigate to events
                    },
                    Calendar {
                        class: "mr-2 h-4 w-4"
                    }
                    "Browse Events"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "w-full justify-start",
                    onclick: move |_| {
                        // Navigate to my events
                    },
                    TrendingUp {
                        class: "mr-2 h-4 w-4"
                    }
                    "My Events"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    class: "w-full justify-start",
                    onclick: move |_| {
                        // Navigate to profile
                    },
                    Users {
                        class: "mr-2 h-4 w-4"
                    }
                    "Edit Profile"
                }
            }
        }
    }
}