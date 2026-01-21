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
use crate::components::dropdown_menu::DropdownMenu;
use crate::components::dialog::Dialog;

use lucide_rust::dioxus::{
    search_icon::Search,
    filter_icon::Filter,
    download_icon::Download,
    more_horizontal_icon::MoreHorizontal,
    mail_icon::Mail,
    phone_icon::Phone,
    calendar_icon::Calendar,
    credit_card_icon::CreditCard,
    user_check_icon::UserCheck,
    user_x_icon::UserX,
    eye_icon::Eye,
};

#[component]
pub fn MembersPage() -> Element {
    let auth = use_auth();
    let mut search_query = use_signal(String::new);
    let mut filter_status = use_signal(|| String::from("all"));
    let mut selected_member = use_signal::<Option<String>>(|| None);

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

    // Mock data - members
    let all_members = vec![
        ("1", "John", "Doe", "john@example.com", "555-1234", "MEM-001", true, true, 5),
        ("2", "Jane", "Smith", "jane@example.com", "555-5678", "MEM-002", true, false, 3),
        ("3", "Bob", "Johnson", "bob@example.com", "555-9012", "MEM-003", false, true, 0),
        ("4", "Alice", "Williams", "alice@example.com", "555-3456", "MEM-004", true, true, 8),
        ("5", "Charlie", "Brown", "charlie@example.com", "555-7890", "MEM-005", true, false, 2),
    ];

    let total_members = all_members.len();
    let active_members = all_members.iter().filter(|m| m.6).count();
    let with_rfid = all_members.iter().filter(|m| m.7).count();
    let new_this_month = 2; // Mock value

    rsx! {
        div {
            class: "container mx-auto py-8 px-4 md:px-6 max-w-7xl",
            PageHeader {
                title: "Member Management",
                description: "View and manage all registered members"
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
                        placeholder: "Search members by name, email, or member ID...",
                        value: search_query(),
                        onchange: move |val: String| {
                            search_query.set(val);
                        },
                    }
                }
                
                Select {
                    options: vec![
                        ("all", "All Members"),
                        ("active", "Active"),
                        ("inactive", "Inactive"),
                        ("has-rfid", "Has RFID Card"),
                        ("no-rfid", "No RFID Card"),
                    ],
                    value: filter_status(),
                    onchange: move |val: String| {
                        filter_status.set(val);
                    },
                }
            }

            // Stats Summary
            div {
                class: "grid grid-cols-1 sm:grid-cols-4 gap-4 mb-8",
                MembersStatsCard {
                    icon_bg: "bg-blue-100",
                    icon_color: "text-blue-600",
                    title: total_members.to_string(),
                    subtitle: "Total Members",
                    icon: rsx! {
                        UserCheck {
                            class: "h-5 w-5 text-blue-600"
                        }
                    }
                }
                MembersStatsCard {
                    icon_bg: "bg-green-100",
                    icon_color: "text-green-600",
                    title: active_members.to_string(),
                    subtitle: "Active Members",
                    icon: rsx! {
                        UserCheck {
                            class: "h-5 w-5 text-green-600"
                        }
                    }
                }
                MembersStatsCard {
                    icon_bg: "bg-purple-100",
                    icon_color: "text-purple-600",
                    title: with_rfid.to_string(),
                    subtitle: "With RFID Cards",
                    icon: rsx! {
                        CreditCard {
                            class: "h-5 w-5 text-purple-600"
                        }
                    }
                }
                MembersStatsCard {
                    icon_bg: "bg-orange-100",
                    icon_color: "text-orange-600",
                    title: new_this_month.to_string(),
                    subtitle: "New This Month",
                    icon: rsx! {
                        Calendar {
                            class: "h-5 w-5 text-orange-600"
                        }
                    }
                }
            }

            // Members Table
            if all_members.is_empty() {
                rsx! {
                    Card {
                        class: "text-center py-12",
                        CardContent {
                            div {
                                class: "text-gray-400 mb-4",
                                UserCheck {
                                    class: "h-12 w-12 mx-auto"
                                }
                            }
                            h3 {
                                class: "text-lg font-semibold text-gray-900 mb-2",
                                "No members found"
                            }
                            p {
                                class: "text-gray-600",
                                "No members match your search or filter"
                            }
                        }
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
                                        TableHead { "Member" }
                                        TableHead { "Member ID" }
                                        TableHead { "Email" }
                                        TableHead { "Join Date" }
                                        TableHead { "Status" }
                                        TableHead { "RFID" }
                                        TableHead { "Events" }
                                        TableHead {
                                            class: "text-right",
                                            "Actions"
                                        }
                                    }
                                }
                                TableBody {
                                    {all_members.iter().map(|member| {
                                        let (id, first_name, last_name, email, phone, member_id, is_active, has_rfid, event_count) = member;
                                        rsx! {
                                            TableRow {
                                                key: "{id}",
                                                TableCell {
                                                    div {
                                                        class: "flex items-center space-x-3",
                                                        div {
                                                            class: "w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-medium",
                                                            "{first_name.chars().next().unwrap()}{last_name.chars().next().unwrap()}"
                                                        }
                                                        div {
                                                            p {
                                                                class: "font-medium",
                                                                "{first_name} {last_name}"
                                                            }
                                                            p {
                                                                class: "text-sm text-gray-500",
                                                                "{email}"
                                                            }
                                                        }
                                                    }
                                                }
                                                TableCell {
                                                    span {
                                                        class: "font-mono text-sm",
                                                        "{member_id}"
                                                    }
                                                }
                                                TableCell {
                                                    div {
                                                        class: "space-y-1",
                                                        div {
                                                            class: "flex items-center text-sm",
                                                            Mail {
                                                                class: "h-3 w-3 mr-1 text-gray-400"
                                                            }
                                                            span {
                                                                class: "truncate max-w-[120px]",
                                                                "{email}"
                                                            }
                                                        }
                                                        if !phone.is_empty() {
                                                            div {
                                                                class: "flex items-center text-sm text-gray-600",
                                                                Phone {
                                                                    class: "h-3 w-3 mr-1 text-gray-400"
                                                                }
                                                                span {
                                                                    "{phone}"
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                TableCell {
                                                    "Jan 15, 2026"
                                                }
                                                TableCell {
                                                    Badge {
                                                        variant: if *is_active { BadgeVariant::Outline } else { BadgeVariant::Secondary },
                                                        class: if *is_active { "border-green-200 text-green-700" } else { "border-red-200 text-red-700" },
                                                        if *is_active { "Active" } else { "Inactive" }
                                                    }
                                                }
                                                TableCell {
                                                    Badge {
                                                        variant: BadgeVariant::Outline,
                                                        class: if *has_rfid { "border-blue-200 text-blue-700" } else { "border-gray-200 text-gray-700" },
                                                        if *has_rfid { "Active" } else { "None" }
                                                    }
                                                }
                                                TableCell {
                                                    "{event_count}"
                                                }
                                                TableCell {
                                                    class: "text-right",
                                                    Button {
                                                        variant: ButtonVariant::Ghost,
                                                        size: ButtonSize::Sm,
                                                        onclick: move |_| {
                                                            selected_member.set(Some(id.to_string()));
                                                        },
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

            // Member Details Dialog
            if let Some(member_id) = selected_member() {
                rsx! {
                    MemberDetailsDialog {
                        member_id: member_id.clone(),
                        onclose: move |_| {
                            selected_member.set(None);
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MembersStatsCard(icon_bg: String, icon_color: String, title: String, subtitle: String, icon: Element) -> Element {
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
                        "{title}"
                    }
                    p {
                        class: "text-xs text-gray-600",
                        "{subtitle}"
                    }
                }
            }
        }
    }
}

#[component]
fn MemberDetailsDialog(member_id: String, onclose: EventHandler<()>) -> Element {
    // Mock member data
    let (first_name, last_name, email, phone) = ("John", "Doe", "john@example.com", "555-1234");
    let member_id_str = "MEM-001";
    let is_active = true;

    rsx! {
        Dialog {
            div {
                class: "fixed inset-0 z-50 bg-black/50 flex items-center justify-center",
                div {
                    class: "bg-white rounded-lg shadow-lg max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                    div {
                        class: "p-6 border-b",
                        h2 {
                            class: "text-lg font-semibold",
                            "Member Details"
                        }
                        p {
                            class: "text-sm text-gray-600",
                            "Detailed information for {first_name} {last_name}"
                        }
                    }
                    div {
                        class: "p-6 space-y-6",
                        // Basic Info
                        div {
                            class: "grid grid-cols-2 gap-4",
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Name"
                                }
                                p {
                                    class: "text-sm",
                                    "{first_name} {last_name}"
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Member ID"
                                }
                                p {
                                    class: "text-sm font-mono",
                                    "{member_id_str}"
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Email"
                                }
                                p {
                                    class: "text-sm",
                                    "{email}"
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Phone"
                                }
                                p {
                                    class: "text-sm",
                                    "{phone}"
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Join Date"
                                }
                                p {
                                    class: "text-sm",
                                    "January 15, 2026"
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Status"
                                }
                                Badge {
                                    variant: if is_active { BadgeVariant::Outline } else { BadgeVariant::Secondary },
                                    class: if is_active { "border-green-200 text-green-700" } else { "border-red-200 text-red-700" },
                                    if is_active { "Active" } else { "Inactive" }
                                }
                            }
                        }

                        // RFID Cards
                        div {
                            label {
                                class: "text-sm font-medium text-gray-500 mb-2 block",
                                "RFID Cards"
                            }
                            div {
                                class: "p-3 bg-gray-50 rounded-lg",
                                div {
                                    class: "flex items-center justify-between",
                                    div {
                                        p {
                                            class: "text-sm font-medium",
                                            "Card #CARD-001"
                                        }
                                        p {
                                            class: "text-xs text-gray-500",
                                            "Issued: January 15, 2026"
                                        }
                                    }
                                    Badge {
                                        variant: BadgeVariant::Outline,
                                        class: "border-green-200 text-green-700",
                                        "Active"
                                    }
                                }
                            }
                        }

                        // Event Statistics
                        div {
                            label {
                                class: "text-sm font-medium text-gray-500 mb-2 block",
                                "Event Statistics"
                            }
                            div {
                                class: "grid grid-cols-2 gap-4",
                                div {
                                    class: "p-3 bg-gray-50 rounded-lg",
                                    p {
                                        class: "text-sm font-medium",
                                        "Events Registered"
                                    }
                                    p {
                                        class: "text-lg font-bold",
                                        "5"
                                    }
                                }
                                div {
                                    class: "p-3 bg-gray-50 rounded-lg",
                                    p {
                                        class: "text-sm font-medium",
                                        "Last Activity"
                                    }
                                    p {
                                        class: "text-sm",
                                        "January 20, 2026"
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