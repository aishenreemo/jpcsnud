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
use crate::components::textarea::Textarea;

use lucide_rust::dioxus::{
    search_icon::Search,
    filter_icon::Filter,
    download_icon::Download,
    more_horizontal_icon::MoreHorizontal,
    mail_icon::Mail,
    map_pin_icon::MapPin,
    clock_icon::Clock,
    circle_check_icon::CircleCheck,
    circle_x_icon::CircleX,
    circle_alert_icon::CircleAlert,
    package_icon::Package,
    truck_icon::Truck,
    eye_icon::Eye,
};

#[derive(Clone, PartialEq, Debug)]
struct RfidOrder {
    id: String,
    member_name: String,
    member_email: String,
    request_date: String,
    status: String,
    reason: String,
    address: String,
    notes: Option<String>,
}

#[component]
pub fn RfidOrdersPage() -> Element {
    let auth = use_auth();
    let mut search_query = use_signal(String::new);
    let mut filter_status = use_signal(|| String::from("all"));
    let mut selected_order = use_signal::<Option<String>>(|| None);
    let mut action_type = use_signal::<Option<String>>(|| None);
    let mut action_note = use_signal(String::new);
    let mut is_processing = use_signal(|| false);

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

    // Mock RFID orders data
    let all_orders = vec![
        RfidOrder {
            id: "ORD-001".to_string(),
            member_name: "John Doe".to_string(),
            member_email: "john@example.com".to_string(),
            request_date: "2026-01-15".to_string(),
            status: "pending".to_string(),
            reason: "Initial card request".to_string(),
            address: "123 Main St, New York, NY".to_string(),
            notes: None,
        },
        RfidOrder {
            id: "ORD-002".to_string(),
            member_name: "Jane Smith".to_string(),
            member_email: "jane@example.com".to_string(),
            request_date: "2026-01-10".to_string(),
            status: "approved".to_string(),
            reason: "Replacement card".to_string(),
            address: "456 Oak Ave, Los Angeles, CA".to_string(),
            notes: Some("Approved on 2026-01-12".to_string()),
        },
        RfidOrder {
            id: "ORD-003".to_string(),
            member_name: "Bob Johnson".to_string(),
            member_email: "bob@example.com".to_string(),
            request_date: "2026-01-08".to_string(),
            status: "shipped".to_string(),
            reason: "New membership card".to_string(),
            address: "789 Pine Rd, Chicago, IL".to_string(),
            notes: Some("Tracking: TRK123456".to_string()),
        },
        RfidOrder {
            id: "ORD-004".to_string(),
            member_name: "Alice Williams".to_string(),
            member_email: "alice@example.com".to_string(),
            request_date: "2026-01-05".to_string(),
            status: "delivered".to_string(),
            reason: "Initial card request".to_string(),
            address: "321 Elm St, Houston, TX".to_string(),
            notes: Some("Delivered on 2026-01-18".to_string()),
        },
        RfidOrder {
            id: "ORD-005".to_string(),
            member_name: "Charlie Brown".to_string(),
            member_email: "charlie@example.com".to_string(),
            request_date: "2026-01-12".to_string(),
            status: "rejected".to_string(),
            reason: "Duplicate request".to_string(),
            address: "654 Cedar Ln, Phoenix, AZ".to_string(),
            notes: Some("Rejected: Existing active card".to_string()),
        },
    ];

    let pending_count = all_orders.iter().filter(|o| o.status == "pending").count();
    let approved_count = all_orders.iter().filter(|o| o.status == "approved").count();
    let produced_count = all_orders.iter().filter(|o| o.status == "produced").count();
    let shipped_count = all_orders.iter().filter(|o| o.status == "shipped").count();
    let delivered_count = all_orders.iter().filter(|o| o.status == "delivered").count();

    rsx! {
        div {
            class: "container mx-auto py-8 px-4 md:px-6 max-w-7xl",
            PageHeader {
                title: "RFID Card Orders",
                description: "Manage member RFID card requests and orders"
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
                        placeholder: "Search orders by member name, email, or order ID...",
                        value: search_query(),
                        onchange: move |val: String| {
                            search_query.set(val);
                        },
                    }
                }
                
                Select {
                    options: vec![
                        ("all", "All Orders"),
                        ("pending", "Pending"),
                        ("approved", "Approved"),
                        ("rejected", "Rejected"),
                        ("produced", "Produced"),
                        ("shipped", "Shipped"),
                        ("delivered", "Delivered"),
                    ],
                    value: filter_status(),
                    onchange: move |val: String| {
                        filter_status.set(val);
                    },
                }
            }

            // Stats Summary
            div {
                class: "grid grid-cols-1 sm:grid-cols-5 gap-4 mb-8",
                RfidStatsCard {
                    icon_bg: "bg-yellow-100",
                    icon_color: "text-yellow-600",
                    count: pending_count.to_string(),
                    label: "Pending",
                    icon: rsx! {
                        Clock {
                            class: "h-5 w-5 text-yellow-600"
                        }
                    }
                }
                RfidStatsCard {
                    icon_bg: "bg-blue-100",
                    icon_color: "text-blue-600",
                    count: approved_count.to_string(),
                    label: "Approved",
                    icon: rsx! {
                        CircleCheck {
                            class: "h-5 w-5 text-blue-600"
                        }
                    }
                }
                RfidStatsCard {
                    icon_bg: "bg-purple-100",
                    icon_color: "text-purple-600",
                    count: produced_count.to_string(),
                    label: "Produced",
                    icon: rsx! {
                        Package {
                            class: "h-5 w-5 text-purple-600"
                        }
                    }
                }
                RfidStatsCard {
                    icon_bg: "bg-indigo-100",
                    icon_color: "text-indigo-600",
                    count: shipped_count.to_string(),
                    label: "Shipped",
                    icon: rsx! {
                        Truck {
                            class: "h-5 w-5 text-indigo-600"
                        }
                    }
                }
                RfidStatsCard {
                    icon_bg: "bg-green-100",
                    icon_color: "text-green-600",
                    count: delivered_count.to_string(),
                    label: "Delivered",
                    icon: rsx! {
                        CircleCheck {
                            class: "h-5 w-5 text-green-600"
                        }
                    }
                }
            }

            // Orders Table
            if all_orders.is_empty() {
                rsx! {
                    Card {
                        class: "text-center py-12",
                        CardContent {
                            div {
                                class: "text-gray-400 mb-4",
                                Package {
                                    class: "h-12 w-12 mx-auto"
                                }
                            }
                            h3 {
                                class: "text-lg font-semibold text-gray-900 mb-2",
                                "No orders found"
                            }
                            p {
                                class: "text-gray-600",
                                "No RFID card orders have been placed yet"
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
                                        TableHead { "Order ID" }
                                        TableHead { "Request Date" }
                                        TableHead { "Status" }
                                        TableHead { "Reason" }
                                        TableHead { "Address" }
                                        TableHead {
                                            class: "text-right",
                                            "Actions"
                                        }
                                    }
                                }
                                TableBody {
                                    {all_orders.iter().map(|order| {
                                        let order_clone = order.clone();
                                        rsx! {
                                            TableRow {
                                                key: "{order.id}",
                                                TableCell {
                                                    div {
                                                        class: "flex items-center space-x-3",
                                                        div {
                                                            class: "w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-medium",
                                                            "{order.member_name.chars().next().unwrap()}"
                                                        }
                                                        div {
                                                            p {
                                                                class: "font-medium",
                                                                "{order.member_name}"
                                                            }
                                                            p {
                                                                class: "text-sm text-gray-500",
                                                                "{order.member_email}"
                                                            }
                                                        }
                                                    }
                                                }
                                                TableCell {
                                                    span {
                                                        class: "font-mono text-sm",
                                                        "{order.id}"
                                                    }
                                                }
                                                TableCell {
                                                    "{order.request_date}"
                                                }
                                                TableCell {
                                                    Badge {
                                                        variant: BadgeVariant::Outline,
                                                        class: get_status_badge_class(&order.status),
                                                        "{order.status}"
                                                    }
                                                }
                                                TableCell {
                                                    span {
                                                        class: "max-w-[200px] truncate",
                                                        "{order.reason}"
                                                    }
                                                }
                                                TableCell {
                                                    span {
                                                        class: "max-w-[200px] truncate",
                                                        "{order.address}"
                                                    }
                                                }
                                                TableCell {
                                                    class: "text-right",
                                                    Button {
                                                        variant: ButtonVariant::Ghost,
                                                        size: ButtonSize::Sm,
                                                        onclick: move |_| {
                                                            selected_order.set(Some(order_clone.id.clone()));
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

            // Order Details Dialog
            if let Some(order_id) = selected_order() {
                if let Some(order) = all_orders.iter().find(|o| o.id == order_id) {
                    let order_clone = order.clone();
                    rsx! {
                        RfidOrderDialog {
                            order: order_clone,
                            action_type: action_type(),
                            action_note: action_note(),
                            is_processing: is_processing(),
                            onclose: move |_| {
                                selected_order.set(None);
                                action_type.set(None);
                                action_note.set(String::new());
                            },
                            onaction: move |action: String| {
                                action_type.set(Some(action));
                            },
                            onnote_change: move |note: String| {
                                action_note.set(note);
                            },
                            onsubmit: move |_| {
                                is_processing.set(true);
                                // Simulate API call
                                spawn(async move {
                                    async_std::task::sleep(std::time::Duration::from_millis(1500)).await;
                                    is_processing.set(false);
                                    selected_order.set(None);
                                    action_type.set(None);
                                    action_note.set(String::new());
                                });
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RfidStatsCard(icon_bg: String, icon_color: String, count: String, label: String, icon: Element) -> Element {
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
fn RfidOrderDialog(
    order: RfidOrder,
    action_type: Option<String>,
    action_note: String,
    is_processing: bool,
    onclose: EventHandler<()>,
    onaction: EventHandler<String>,
    onnote_change: EventHandler<String>,
    onsubmit: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/50 flex items-center justify-center",
            div {
                class: "bg-white rounded-lg shadow-lg max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto",
                div {
                    class: "p-6 border-b",
                    h2 {
                        class: "text-lg font-semibold",
                        if let Some(ref action) = action_type {
                            "{action} Order"
                        } else {
                            "Order Details"
                        }
                    }
                    p {
                        class: "text-sm text-gray-600",
                        "Order #{order.id} for {order.member_name}"
                    }
                }
                div {
                    class: "p-6 space-y-6",
                    if action_type.is_none() {
                        // View details mode
                        rsx! {
                            div {
                                class: "grid grid-cols-2 gap-4",
                                div {
                                    label {
                                        class: "text-sm font-medium text-gray-500",
                                        "Member"
                                    }
                                    p {
                                        class: "text-sm",
                                        "{order.member_name}"
                                    }
                                    p {
                                        class: "text-xs text-gray-500",
                                        "{order.member_email}"
                                    }
                                }
                                div {
                                    label {
                                        class: "text-sm font-medium text-gray-500",
                                        "Order ID"
                                    }
                                    p {
                                        class: "text-sm font-mono",
                                        "{order.id}"
                                    }
                                }
                            }
                            div {
                                class: "grid grid-cols-2 gap-4",
                                div {
                                    label {
                                        class: "text-sm font-medium text-gray-500",
                                        "Request Date"
                                    }
                                    p {
                                        class: "text-sm",
                                        "{order.request_date}"
                                    }
                                }
                                div {
                                    label {
                                        class: "text-sm font-medium text-gray-500",
                                        "Status"
                                    }
                                    Badge {
                                        variant: BadgeVariant::Outline,
                                        class: get_status_badge_class(&order.status),
                                        "{order.status}"
                                    }
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500",
                                    "Reason"
                                }
                                p {
                                    class: "text-sm",
                                    "{order.reason}"
                                }
                            }
                            div {
                                label {
                                    class: "text-sm font-medium text-gray-500 mb-2 block",
                                    "Shipping Address"
                                }
                                div {
                                    class: "p-3 bg-gray-50 rounded-lg",
                                    p {
                                        class: "text-sm",
                                        "{order.address}"
                                    }
                                }
                            }
                            if let Some(ref notes) = order.notes {
                                div {
                                    label {
                                        class: "text-sm font-medium text-gray-500",
                                        "Notes"
                                    }
                                    p {
                                        class: "text-sm",
                                        "{notes}"
                                    }
                                }
                            }
                        }
                    } else {
                        // Action mode
                        rsx! {
                            div {
                                class: "p-4 bg-blue-50 border border-blue-200 rounded-lg",
                                p {
                                    class: "text-sm text-blue-800",
                                    "You are about to {action_type.as_ref().unwrap()} this RFID card order."
                                }
                            }
                            div {
                                class: "space-y-2",
                                label {
                                    class: "text-sm font-medium text-gray-700",
                                    if action_type.as_ref().map(|a| a.as_str()) == Some("approve") {
                                        "Approval Note"
                                    } else if action_type.as_ref().map(|a| a.as_str()) == Some("reject") {
                                        "Rejection Reason"
                                    } else {
                                        "Shipping Note"
                                    }
                                    " (Optional)"
                                }
                                Textarea {
                                    value: action_note.clone(),
                                    onchange: move |val: String| {
                                        onnote_change.call(val);
                                    },
                                    placeholder: "Enter details...",
                                }
                            }
                        }
                    }
                }
                div {
                    class: "p-6 border-t flex gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            onclose.call(());
                        },
                        "Close"
                    }
                    if action_type.is_none() {
                        rsx! {
                            Button {
                                variant: ButtonVariant::Default,
                                onclick: move |_| {
                                    onaction.call("approve".to_string());
                                },
                                "Approve"
                            }
                            Button {
                                variant: ButtonVariant::Default,
                                onclick: move |_| {
                                    onaction.call("ship".to_string());
                                },
                                "Mark as Shipped"
                            }
                        }
                    } else {
                        rsx! {
                            Button {
                                variant: ButtonVariant::Default,
                                disabled: is_processing,
                                onclick: move |_| {
                                    onsubmit.call(());
                                },
                                if is_processing { "Processing..." } else { "Confirm" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_status_badge_class(status: &str) -> &'static str {
    match status {
        "pending" => "border-yellow-200 text-yellow-700",
        "approved" => "border-blue-200 text-blue-700",
        "produced" => "border-purple-200 text-purple-700",
        "shipped" => "border-indigo-200 text-indigo-700",
        "delivered" => "border-green-200 text-green-700",
        "rejected" => "border-red-200 text-red-700",
        _ => "border-gray-200 text-gray-700",
    }
}