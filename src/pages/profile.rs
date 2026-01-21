use dioxus::prelude::*;
use crate::contexts::use_auth;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::badge::{Badge, BadgeVariant};
use crate::components::page_header::PageHeader;
use crate::components::separator::Separator;

use lucide_rust::dioxus::{
    user_icon::User,
    mail_icon::Mail,
    phone_icon::Phone,
    calendar_icon::Calendar,
    credit_card_icon::CreditCard,
    check_circle_icon::CheckCircle,
    alert_circle_icon::AlertCircle,
};

#[derive(Clone)]
struct FormData {
    first_name: String,
    last_name: String,
    phone: String,
}

#[component]
pub fn Profile() -> Element {
    let auth = use_auth();
    let mut is_editing = use_signal(false);
    let mut is_saving = use_signal(false);
    let mut form_data = use_signal(FormData {
        first_name: String::new(),
        last_name: String::new(),
        phone: String::new(),
    });

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

    let handle_save = move |_| {
        is_saving.set(true);
        spawn {
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
            is_saving.set(false);
            is_editing.set(false);
        }
    };

    let handle_cancel = move |_| {
        is_editing.set(false);
    };

    rsx! {
        div {
            class: "container mx-auto py-6 sm:py-8 px-3 sm:px-4 md:px-6 max-w-4xl",
            PageHeader {
                title: "Profile Settings",
                description: "Manage your account information and preferences"
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6 sm:gap-8",
                // Main Profile Form
                div {
                    class: "lg:col-span-2",
                    Card {
                        CardHeader {
                            class: "flex flex-col space-y-2 sm:flex-row sm:items-center sm:justify-between sm:space-y-0",
                            div {
                                CardTitle { "Personal Information" }
                                CardDescription {
                                    "Update your personal details and contact information"
                                }
                            }
                            if !is_editing() {
                                Button {
                                    variant: ButtonVariant::Outline,
                                    onclick: move |_| is_editing.set(true),
                                    "Edit Profile"
                                }
                            }
                        }
                        CardContent {
                            class: "space-y-6",
                            // First and Last Name
                            div {
                                class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                div {
                                    class: "space-y-2",
                                    Label {
                                        label_for: "firstName",
                                        "First Name"
                                    }
                                    if is_editing() {
                                        Input {
                                            id: "firstName",
                                            name: "firstName",
                                            placeholder: "Enter your first name",
                                            value: form_data().first_name,
                                            onchange: move |evt| {
                                                let mut fd = form_data();
                                                fd.first_name = evt.value();
                                                form_data.set(fd);
                                            }
                                        }
                                    } else {
                                        div {
                                            class: "flex items-center space-x-2 py-2",
                                            User {
                                                class: "h-4 w-4 text-gray-500"
                                            }
                                            span { "John Doe" }
                                        }
                                    }
                                }

                                div {
                                    class: "space-y-2",
                                    Label {
                                        label_for: "lastName",
                                        "Last Name"
                                    }
                                    if is_editing() {
                                        Input {
                                            id: "lastName",
                                            name: "lastName",
                                            placeholder: "Enter your last name",
                                            value: form_data().last_name,
                                            onchange: move |evt| {
                                                let mut fd = form_data();
                                                fd.last_name = evt.value();
                                                form_data.set(fd);
                                            }
                                        }
                                    } else {
                                        div {
                                            class: "flex items-center space-x-2 py-2",
                                            User {
                                                class: "h-4 w-4 text-gray-500"
                                            }
                                            span { "Doe" }
                                        }
                                    }
                                }
                            }

                            // Email
                            div {
                                class: "space-y-2",
                                Label {
                                    label_for: "email",
                                    "Email Address"
                                }
                                div {
                                    class: "flex items-center space-x-2 py-2 text-gray-500",
                                    Mail {
                                        class: "h-4 w-4"
                                    }
                                    span { "john@example.com" }
                                    Badge {
                                        variant: BadgeVariant::Outline,
                                        class: "ml-auto",
                                        "Cannot be changed"
                                    }
                                }
                            }

                            // Phone
                            div {
                                class: "space-y-2",
                                Label {
                                    label_for: "phone",
                                    "Phone Number"
                                }
                                if is_editing() {
                                    Input {
                                        id: "phone",
                                        name: "phone",
                                        placeholder: "Enter your phone number",
                                        input_type: "tel",
                                        value: form_data().phone,
                                        onchange: move |evt| {
                                            let mut fd = form_data();
                                            fd.phone = evt.value();
                                            form_data.set(fd);
                                        }
                                    }
                                } else {
                                    div {
                                        class: "flex items-center space-x-2 py-2",
                                        Phone {
                                            class: "h-4 w-4 text-gray-500"
                                        }
                                        span { "+1-555-0123" }
                                    }
                                }
                            }

                            // Save/Cancel Buttons
                            if is_editing() {
                                div {
                                    class: "flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-4 pt-4",
                                    Button {
                                        onclick: handle_save,
                                        disabled: is_saving(),
                                        if is_saving() { "Saving..." } else { "Save Changes" }
                                    }
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        onclick: handle_cancel,
                                        "Cancel"
                                    }
                                }
                            }
                        }
                    }
                }

                // Sidebar
                div {
                    class: "space-y-4 sm:space-y-6",
                    // Account Status
                    Card {
                        CardHeader {
                            CardTitle { "Account Status" }
                        }
                        CardContent {
                            class: "space-y-3 sm:space-y-4",
                            div {
                                class: "flex items-center justify-between",
                                span {
                                    class: "text-sm text-gray-600",
                                    "Account Type"
                                }
                                Badge {
                                    variant: BadgeVariant::Outline,
                                    class: "capitalize",
                                    "member"
                                }
                            }

                            div {
                                class: "flex items-center justify-between",
                                span {
                                    class: "text-sm text-gray-600",
                                    "Status"
                                }
                                Badge {
                                    variant: BadgeVariant::Outline,
                                    class: "text-green-600 border-green-200",
                                    "Active"
                                }
                            }

                            div {
                                class: "flex items-start justify-between",
                                span {
                                    class: "text-sm text-gray-600",
                                    "Member Since"
                                }
                                div {
                                    class: "text-right",
                                    div {
                                        class: "flex items-center space-x-1",
                                        Calendar {
                                            class: "h-4 w-4 text-gray-500"
                                        }
                                        span {
                                            class: "text-sm",
                                            "January 15, 2026"
                                        }
                                    }
                                }
                            }

                            div {
                                class: "flex items-center justify-between",
                                span {
                                    class: "text-sm text-gray-600",
                                    "Membership ID"
                                }
                                span {
                                    class: "text-sm font-mono",
                                    "MEM-20260115-0001"
                                }
                            }
                        }
                    }

                    // RFID Card Status
                    Card {
                        CardHeader {
                            CardTitle { "RFID Card" }
                            CardDescription { "Your digital membership card" }
                        }
                        CardContent {
                            class: "space-y-4",
                            div {
                                class: "p-3 border rounded-lg bg-green-50 border-green-200",
                                div {
                                    class: "flex items-center space-x-2",
                                    CheckCircle {
                                        class: "h-5 w-5 text-green-600"
                                    }
                                    span {
                                        class: "font-medium capitalize text-green-800",
                                        "Active"
                                    }
                                }
                                p {
                                    class: "text-sm mt-1 text-green-700",
                                    "Card XXXX-XXXX-XXXX-1234"
                                }
                                p {
                                    class: "text-xs mt-1 text-green-700",
                                    "Issued: January 15, 2026"
                                }
                            }

                            div {
                                class: "space-y-2",
                                Button {
                                    variant: ButtonVariant::Outline,
                                    size: ButtonSize::Sm,
                                    class: "w-full justify-start",
                                    CreditCard {
                                        class: "mr-2 h-4 w-4"
                                    }
                                    "Manage Card"
                                }
                            }
                        }
                    }

                    // Quick Actions
                    Card {
                        CardHeader {
                            CardTitle { "Quick Actions" }
                        }
                        CardContent {
                            class: "space-y-2",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                class: "w-full justify-start text-red-600 hover:text-red-700 hover:bg-red-50",
                                "Deactivate Account"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                class: "w-full justify-start text-blue-600 hover:text-blue-700 hover:bg-blue-50",
                                "Download My Data"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                class: "w-full justify-start text-gray-600 hover:text-gray-700 hover:bg-gray-50",
                                "Privacy Settings"
                            }
                        }
                    }
                }
            }
        }
    }
}