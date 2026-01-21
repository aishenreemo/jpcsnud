use dioxus::prelude::*;
use crate::contexts::use_auth;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::textarea::Textarea;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::page_header::PageHeader;
use crate::components::separator::Separator;

use lucide_rust::dioxus::{
    check_circle_icon::CheckCircle,
    package_icon::Package,
    user_icon::User,
    phone_icon::Phone,
};

#[derive(Clone, Copy, Debug)]
enum OrderStatus {
    Pending,
    Approved,
    Produced,
    Shipped,
    Delivered,
    Rejected,
}

#[derive(Clone)]
struct FormData {
    reason: String,
    street: String,
    city: String,
    state: String,
    zip_code: String,
    country: String,
    notes: String,
}

#[component]
pub fn RfidOrder() -> Element {
    let auth = use_auth();
    let mut form_data = use_signal(|| FormData {
        reason: String::new(),
        street: String::new(),
        city: String::new(),
        state: String::new(),
        zip_code: String::new(),
        country: "USA".to_string(),
        notes: String::new(),
    });
    let mut is_ordering = use_signal(|| false);

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
            class: "container mx-auto py-8 px-4 md:px-6 max-w-4xl",
            PageHeader {
                title: "RFID Card Management",
                description: "Request and manage your RFID membership card"
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                div {
                    class: "lg:col-span-2",
                    CurrentStatusCard {}
                    OrderFormCard {
                        form_data: form_data,
                        is_ordering: is_ordering
                    }
                    OrderHistoryCard {}
                }

                div {
                    class: "space-y-6",
                    SupportCard {}
                    FaqCard {}
                }
            }
        }
    }
}

#[component]
fn CurrentStatusCard() -> Element {
    rsx! {
        Card {
            class: "mb-6",
            CardHeader {
                CardTitle { "Current RFID Status" }
                CardDescription { "Your current RFID card status and information" }
            }
            CardContent {
                div {
                    class: "p-4 bg-green-50 border border-green-200 rounded-lg",
                    div {
                        class: "flex items-center space-x-3",
                        CheckCircle {
                            class: "h-6 w-6 text-green-600"
                        }
                        div {
                            h3 {
                                class: "font-medium text-green-800",
                                "Active RFID Card"
                            }
                            p {
                                class: "text-sm text-green-700",
                                "Card Number: XXXX-XXXX-XXXX-1234"
                            }
                            p {
                                class: "text-sm text-green-700",
                                "Issued: January 15, 2026"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn OrderFormCard(form_data: Signal<FormData>, is_ordering: Signal<bool>) -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Request RFID Card" }
                CardDescription {
                    "Fill out the form below to request your RFID membership card"
                }
            }
            CardContent {
                form {
                    class: "space-y-6",
                    onsubmit: move |_| {
                        is_ordering.set(true);
                    },

                    div {
                        class: "space-y-2",
                        Label {
                            "Reason for Request *"
                        }
                        select {
                            class: "w-full p-2 border border-gray-300 rounded-md",
                            value: form_data().reason,
                            required: true,
                            option {
                                value: "",
                                "Select a reason"
                            }
                            option {
                                value: "New member card request",
                                "New member card request"
                            }
                            option {
                                value: "Replacement - lost card",
                                "Replacement - lost card"
                            }
                            option {
                                value: "Replacement - damaged card",
                                "Replacement - damaged card"
                            }
                            option {
                                value: "Replacement - stolen card",
                                "Replacement - stolen card"
                            }
                        }
                    }

                    Separator {}

                    div {
                        h3 {
                            class: "text-lg font-medium mb-4",
                            "Shipping Address"
                        }
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                class: "md:col-span-2",
                                Label {
                                    "Street Address *"
                                }
                                Input {
                                    placeholder: "123 Main Street",
                                    value: form_data().street,
                                }
                            }
                            div {
                                Label {
                                    "City *"
                                }
                                Input {
                                    placeholder: "San Francisco",
                                    value: form_data().city,
                                }
                            }
                            div {
                                Label {
                                    "State *"
                                }
                                Input {
                                    placeholder: "CA",
                                    value: form_data().state,
                                }
                            }
                            div {
                                Label {
                                    "ZIP Code *"
                                }
                                Input {
                                    placeholder: "94102",
                                    value: form_data().zip_code,
                                }
                            }
                            div {
                                Label {
                                    "Country *"
                                }
                                select {
                                    class: "w-full p-2 border border-gray-300 rounded-md",
                                    value: form_data().country,
                                    required: true,
                                    option {
                                        value: "USA",
                                        "United States"
                                    }
                                    option {
                                        value: "Canada",
                                        "Canada"
                                    }
                                    option {
                                        value: "Mexico",
                                        "Mexico"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "space-y-2",
                        Label {
                            "Additional Notes"
                        }
                        Textarea {
                            placeholder: "Any special delivery instructions or additional information...",
                            value: form_data().notes,
                        }
                    }

                    div {
                        class: "p-4 bg-blue-50 border border-blue-200 rounded-lg",
                        h4 {
                            class: "font-medium text-blue-800 mb-2",
                            "Important Information"
                        }
                        ul {
                            class: "text-sm text-blue-700 space-y-1",
                            li { "• RFID cards are free for all active members" }
                            li { "• Processing time is typically 3-5 business days" }
                            li { "• Shipping takes 5-7 business days within the US" }
                            li { "• You will receive email updates on your order status" }
                        }
                    }

                    Button {
                        size: ButtonSize::Lg,
                        variant: ButtonVariant::Default,
                        class: "w-full",
                        disabled: is_ordering(),
                        if is_ordering() { "Processing Order..." } else { "Submit RFID Card Request" }
                    }
                }
            }
        }
    }
}

#[component]
fn OrderHistoryCard() -> Element {
    rsx! {
        Card {
            class: "mt-6",
            CardHeader {
                CardTitle { "Order History" }
                CardDescription { "Your previous RFID card orders" }
            }
            CardContent {
                div {
                    class: "space-y-4",
                    div {
                        class: "p-4 border rounded-lg",
                        div {
                            class: "flex items-start justify-between mb-3",
                            div {
                                class: "flex items-center space-x-3",
                                Package {
                                    class: "h-5 w-5 text-gray-600"
                                }
                                div {
                                    p {
                                        class: "font-medium",
                                        "Order #001"
                                    }
                                    p {
                                        class: "text-sm text-gray-600",
                                        "January 15, 2026"
                                    }
                                }
                            }
                            Badge {
                                variant: BadgeVariant::Outline,
                                "produced"
                            }
                        }
                        div {
                            class: "text-sm text-gray-600 space-y-1",
                            p {
                                strong { "Reason: " }
                                "New member card request"
                            }
                            p {
                                strong { "Shipping Address: " }
                                "123 Main Street, San Francisco, CA 94102"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SupportCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Need Help?" }
                CardDescription { "Contact support for assistance" }
            }
            CardContent {
                div {
                    class: "space-y-4",
                    div {
                        class: "flex items-center space-x-3",
                        User {
                            class: "h-5 w-5 text-gray-600"
                        }
                        div {
                            p {
                                class: "text-sm font-medium",
                                "RFID Support Team"
                            }
                            p {
                                class: "text-sm text-gray-600",
                                "support@rfidevents.com"
                            }
                        }
                    }
                    div {
                        class: "flex items-center space-x-3",
                        Phone {
                            class: "h-5 w-5 text-gray-600"
                        }
                        div {
                            p {
                                class: "text-sm font-medium",
                                "Phone Support"
                            }
                            p {
                                class: "text-sm text-gray-600",
                                "1-800-RFID-HELP"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FaqCard() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Frequently Asked Questions" }
            }
            CardContent {
                div {
                    class: "space-y-4",
                    div {
                        h4 {
                            class: "text-sm font-medium mb-1",
                            "How long does delivery take?"
                        }
                        p {
                            class: "text-sm text-gray-600",
                            "5-7 business days within the US, 10-14 days internationally."
                        }
                    }
                    div {
                        h4 {
                            class: "text-sm font-medium mb-1",
                            "What if my card stops working?"
                        }
                        p {
                            class: "text-sm text-gray-600",
                            "Contact support immediately for a replacement card at no charge."
                        }
                    }
                    div {
                        h4 {
                            class: "text-sm font-medium mb-1",
                            "Can I track my order?"
                        }
                        p {
                            class: "text-sm text-gray-600",
                            "Yes, you'll receive email updates and tracking information once shipped."
                        }
                    }
                }
            }
        }
    }
}