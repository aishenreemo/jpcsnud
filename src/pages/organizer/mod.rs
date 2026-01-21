use dioxus::prelude::*;
use crate::contexts::use_auth;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::page_header::PageHeader;

use lucide_rust::dioxus::{
    calendar_icon::Calendar,
    users_icon::Users,
    credit_card_icon::CreditCard,
    trending_up_icon::TrendingUp,
    clock_icon::Clock,
    check_circle_icon::CheckCircle,
    alert_triangle_icon::AlertTriangle,
    plus_icon::Plus,
    bar_chart_3_icon::BarChart3,
    arrow_right_icon::ArrowRight,
    user_check_icon::UserCheck,
};

pub mod events;
pub mod rfid;
pub mod members;

#[component]
pub fn OrganizerDashboard() -> Element {
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
                description: "Your organizer dashboard and system overview"
            }

            // Key Metrics
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6 mb-6 sm:mb-8",
                MetricsCard {
                    icon_bg: "bg-blue-100",
                    icon_color: "text-blue-600",
                    title: "12",
                    subtitle: "Total Events",
                    extra: "8 upcoming",
                    icon: rsx! {
                        Calendar {
                            class: "h-6 w-6 text-blue-600"
                        }
                    }
                }
                MetricsCard {
                    icon_bg: "bg-green-100",
                    icon_color: "text-green-600",
                    title: "245",
                    subtitle: "Total Members",
                    extra: "198 active",
                    icon: rsx! {
                        Users {
                            class: "h-6 w-6 text-green-600"
                        }
                    }
                }
                MetricsCard {
                    icon_bg: "bg-purple-100",
                    icon_color: "text-purple-600",
                    title: "1,240",
                    subtitle: "Event Registrations",
                    extra: "All time",
                    icon: rsx! {
                        TrendingUp {
                            class: "h-6 w-6 text-purple-600"
                        }
                    }
                }
                MetricsCard {
                    icon_bg: "bg-orange-100",
                    icon_color: "text-orange-600",
                    title: "5",
                    subtitle: "Pending RFID Orders",
                    extra: "Need approval",
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
                    UrgentTasksCard {}
                    RecentEventsCard {}
                    RecentRegistrationsCard {}
                }

                // Right Column
                div {
                    class: "space-y-6",
                    QuickActionsCard {}
                    SystemStatusCard {}
                    TodaysEventsCard {}
                }
            }
        }
    }
}

#[component]
fn MetricsCard(icon_bg: String, icon_color: String, title: String, subtitle: String, extra: String, icon: Element) -> Element {
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
                    p {
                        class: "text-xs text-gray-500",
                        "{extra}"
                    }
                }
            }
        }
    }
}

#[component]
fn UrgentTasksCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle {
                    class: "flex items-center",
                    AlertTriangle {
                        class: "h-5 w-5 text-orange-500 mr-2"
                    }
                    "Action Required"
                }
                CardDescription {
                    "Items that need your immediate attention"
                }
            }
            CardContent {
                div {
                    class: "space-y-3",
                    div {
                        class: "flex flex-col sm:flex-row sm:items-center justify-between p-3 border rounded-lg hover:bg-gray-50 gap-2 sm:gap-0",
                        div {
                            Badge {
                                variant: BadgeVariant::Destructive,
                                class: "mb-1",
                                "RFID Orders"
                            }
                            p {
                                class: "text-sm text-gray-700",
                                "5 RFID card orders need approval"
                            }
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                // Navigate to RFID orders
                            },
                            "View "
                            ArrowRight {
                                class: "ml-1 h-4 w-4"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RecentEventsCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                class: "flex flex-row items-center justify-between",
                div {
                    CardTitle { "Recent Events" }
                    CardDescription { "Your latest created events" }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        // Navigate to all events
                    },
                    "View All "
                    ArrowRight {
                        class: "ml-2 h-4 w-4"
                    }
                }
            }
            CardContent {
                div {
                    class: "space-y-4",
                    div {
                        class: "flex items-start space-x-3 sm:space-x-4 p-3 border rounded-lg hover:bg-gray-50",
                        div {
                            class: "p-2 bg-blue-100 rounded-lg",
                            Calendar {
                                class: "h-4 w-4 text-blue-600"
                            }
                        }
                        div {
                            class: "flex-1 min-w-0",
                            h4 {
                                class: "font-medium text-black text-sm sm:text-base line-clamp-1",
                                "Tech Summit 2026"
                            }
                            div {
                                class: "flex items-center text-sm text-gray-600 mt-1",
                                Clock {
                                    class: "mr-1 h-4 w-4"
                                }
                                "Feb 15, 2026"
                            }
                            div {
                                class: "flex items-center text-sm text-gray-600",
                                Users {
                                    class: "mr-1 h-4 w-4"
                                }
                                "45 attendees"
                            }
                        }
                        div {
                            class: "flex flex-col sm:flex-row sm:items-center space-y-2 sm:space-y-0 sm:space-x-2",
                            Badge {
                                variant: BadgeVariant::Outline,
                                class: "border-green-200 text-green-700",
                                "published"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Sm,
                                onclick: move |_| {
                                    // Navigate to check-in
                                },
                                "Check-in"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RecentRegistrationsCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Recent Registrations" }
                CardDescription { "Latest event registrations from members" }
            }
            CardContent {
                div {
                    class: "space-y-3",
                    div {
                        class: "flex items-center space-x-3 p-3 border rounded-lg",
                        div {
                            class: "p-2 bg-green-100 rounded-full",
                            UserCheck {
                                class: "h-4 w-4 text-green-600"
                            }
                        }
                        div {
                            class: "flex-1",
                            p {
                                class: "text-sm",
                                span {
                                    class: "font-medium",
                                    "John Doe"
                                }
                                " registered for "
                                a {
                                    href: "#",
                                    class: "text-blue-600 hover:underline",
                                    "Tech Summit 2026"
                                }
                            }
                            p {
                                class: "text-xs text-gray-500",
                                "January 20, 2026"
                            }
                        }
                    }
                    div {
                        class: "flex items-center space-x-3 p-3 border rounded-lg",
                        div {
                            class: "p-2 bg-green-100 rounded-full",
                            UserCheck {
                                class: "h-4 w-4 text-green-600"
                            }
                        }
                        div {
                            class: "flex-1",
                            p {
                                class: "text-sm",
                                span {
                                    class: "font-medium",
                                    "Jane Smith"
                                }
                                " registered for "
                                a {
                                    href: "#",
                                    class: "text-blue-600 hover:underline",
                                    "Networking Night"
                                }
                            }
                            p {
                                class: "text-xs text-gray-500",
                                "January 19, 2026"
                            }
                        }
                    }
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
                class: "space-y-3",
                Button {
                    size: ButtonSize::Lg,
                    variant: ButtonVariant::Default,
                    class: "w-full justify-start text-sm sm:text-base",
                    onclick: move |_| {
                        // Navigate to create event
                    },
                    Plus {
                        class: "mr-2 h-4 w-4"
                    }
                    "Create New Event"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Lg,
                    class: "w-full justify-start",
                    onclick: move |_| {
                        // Navigate to members
                    },
                    Users {
                        class: "mr-2 h-4 w-4"
                    }
                    "Manage Members"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Lg,
                    class: "w-full justify-start",
                    onclick: move |_| {
                        // Navigate to RFID orders
                    },
                    CreditCard {
                        class: "mr-2 h-4 w-4"
                    }
                    "RFID Orders"
                    Badge {
                        variant: BadgeVariant::Destructive,
                        class: "ml-auto",
                        "5"
                    }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Lg,
                    class: "w-full justify-start",
                    onclick: move |_| {
                        // Navigate to reports
                    },
                    BarChart3 {
                        class: "mr-2 h-4 w-4"
                    }
                    "View Reports"
                }
            }
        }
    }
}

#[component]
fn SystemStatusCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "System Status" }
                CardDescription { "Current system health" }
            }
            CardContent {
                class: "space-y-4",
                StatusItem {
                    label: "Platform Status",
                    status: "Operational"
                }
                StatusItem {
                    label: "RFID System",
                    status: "Online"
                }
                StatusItem {
                    label: "Event Check-ins",
                    status: "Active"
                }
                StatusItem {
                    label: "Member Registration",
                    status: "Available"
                }
            }
        }
    }
}

#[component]
fn StatusItem(label: String, status: String) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between",
            span {
                class: "text-sm text-gray-600",
                "{label}"
            }
            div {
                class: "flex items-center space-x-2",
                div {
                    class: "w-2 h-2 bg-green-500 rounded-full"
                }
                span {
                    class: "text-sm text-green-600",
                    "{status}"
                }
            }
        }
    }
}

#[component]
fn TodaysEventsCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Today's Events" }
                CardDescription { "Events happening today" }
            }
            CardContent {
                div {
                    class: "text-center py-4",
                    Calendar {
                        class: "h-8 w-8 text-gray-300 mx-auto mb-2"
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "No events scheduled for today"
                    }
                }
            }
        }
    }
}