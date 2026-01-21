use dioxus::prelude::*;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::radio_group::{RadioGroup, RadioGroupItem};
use crate::components::checkbox::Checkbox;
use crate::Route;

use lucide_rust::dioxus::{
    eye_icon::Eye,
    eye_off_icon::EyeOff,
    user_check_icon::UserCheck,
    users_icon::Users,
};

#[derive(Clone, Debug, PartialEq)]
enum UserRole {
    Member,
    Organizer,
}

impl UserRole {
    fn as_str(&self) -> &str {
        match self {
            UserRole::Member => "member",
            UserRole::Organizer => "organizer",
        }
    }
}

#[derive(Clone)]
struct FormData {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    role: UserRole,
    agree_to_terms: bool,
}

#[component]
pub fn Register() -> Element {
    let mut form_data = use_signal(FormData {
        first_name: String::new(),
        last_name: String::new(),
        email: String::new(),
        phone: String::new(),
        role: UserRole::Member,
        agree_to_terms: false,
    });
    let mut is_loading = use_signal(false);
    let mut show_password = use_signal(false);

    let handle_submit = move |_| {
        let fd = form_data();
        
        if fd.first_name.is_empty() || fd.last_name.is_empty() || fd.email.is_empty() {
            // Show error toast
            return;
        }

        if !fd.agree_to_terms {
            // Show error toast
            return;
        }

        is_loading.set(true);
        // Simulate API call
        spawn {
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            is_loading.set(false);
        }
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center py-8 sm:py-12 px-3 sm:px-4 md:px-6 lg:px-8 bg-gray-50",
            div {
                class: "max-w-md w-full space-y-6 sm:space-y-8",
                div {
                    class: "text-center",
                    div {
                        class: "mx-auto h-12 w-12 rounded-full bg-black flex items-center justify-center mb-4",
                        span {
                            class: "text-white font-bold text-lg",
                            "R"
                        }
                    }
                    h2 {
                        class: "text-2xl sm:text-3xl font-bold text-black",
                        "Join RFID Events"
                    }
                    p {
                        class: "mt-2 text-sm sm:text-base text-gray-600",
                        "Create your account to start attending events"
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Create Account" }
                        CardDescription {
                            "Fill in your information to get started"
                        }
                    }
                    CardContent {
                        form {
                            class: "space-y-4",
                            onsubmit: handle_submit,

                            // First and Last Name
                            div {
                                class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                div {
                                    class: "space-y-2",
                                    Label {
                                        label_for: "firstName",
                                        "First Name *"
                                    }
                                    Input {
                                        id: "firstName",
                                        name: "firstName",
                                        input_type: "text",
                                        placeholder: "John",
                                        value: form_data().first_name,
                                        onchange: move |evt| {
                                            let mut fd = form_data();
                                            fd.first_name = evt.value();
                                            form_data.set(fd);
                                        },
                                        required: true
                                    }
                                }
                                div {
                                    class: "space-y-2",
                                    Label {
                                        label_for: "lastName",
                                        "Last Name *"
                                    }
                                    Input {
                                        id: "lastName",
                                        name: "lastName",
                                        input_type: "text",
                                        placeholder: "Doe",
                                        value: form_data().last_name,
                                        onchange: move |evt| {
                                            let mut fd = form_data();
                                            fd.last_name = evt.value();
                                            form_data.set(fd);
                                        },
                                        required: true
                                    }
                                }
                            }

                            // Email
                            div {
                                class: "space-y-2",
                                Label {
                                    label_for: "email",
                                    "Email address *"
                                }
                                Input {
                                    id: "email",
                                    name: "email",
                                    input_type: "email",
                                    placeholder: "john@example.com",
                                    value: form_data().email,
                                    onchange: move |evt| {
                                        let mut fd = form_data();
                                        fd.email = evt.value();
                                        form_data.set(fd);
                                    },
                                    required: true
                                }
                            }

                            // Phone
                            div {
                                class: "space-y-2",
                                Label {
                                    label_for: "phone",
                                    "Phone Number"
                                }
                                Input {
                                    id: "phone",
                                    name: "phone",
                                    input_type: "tel",
                                    placeholder: "+1-555-0123",
                                    value: form_data().phone,
                                    onchange: move |evt| {
                                        let mut fd = form_data();
                                        fd.phone = evt.value();
                                        form_data.set(fd);
                                    }
                                }
                            }

                            // Account Type Selection
                            div {
                                class: "space-y-3",
                                Label {
                                    "Account Type *"
                                }
                                div {
                                    class: "space-y-2",
                                    // Member Option
                                    div {
                                        class: "flex items-center space-x-2 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer",
                                        onclick: move |_| {
                                            let mut fd = form_data();
                                            fd.role = UserRole::Member;
                                            form_data.set(fd);
                                        },
                                        input {
                                            input_type: "radio",
                                            name: "role",
                                            value: "member",
                                            checked: form_data().role == UserRole::Member,
                                            class: "w-4 h-4"
                                        }
                                        div {
                                            class: "flex items-center space-x-3 flex-1",
                                            Users {
                                                class: "h-5 w-5 text-blue-500"
                                            }
                                            div {
                                                div {
                                                    class: "font-medium",
                                                    "Member"
                                                }
                                                div {
                                                    class: "text-sm text-gray-500",
                                                    "Join events and manage your profile"
                                                }
                                            }
                                        }
                                    }

                                    // Organizer Option
                                    div {
                                        class: "flex items-center space-x-2 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer",
                                        onclick: move |_| {
                                            let mut fd = form_data();
                                            fd.role = UserRole::Organizer;
                                            form_data.set(fd);
                                        },
                                        input {
                                            input_type: "radio",
                                            name: "role",
                                            value: "organizer",
                                            checked: form_data().role == UserRole::Organizer,
                                            class: "w-4 h-4"
                                        }
                                        div {
                                            class: "flex items-center space-x-3 flex-1",
                                            UserCheck {
                                                class: "h-5 w-5 text-purple-500"
                                            }
                                            div {
                                                div {
                                                    class: "font-medium",
                                                    "Organizer"
                                                }
                                                div {
                                                    class: "text-sm text-gray-500",
                                                    "Create and manage events"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Terms and Conditions
                            div {
                                class: "flex items-center space-x-2",
                                input {
                                    input_type: "checkbox",
                                    id: "agreeToTerms",
                                    checked: form_data().agree_to_terms,
                                    onchange: move |evt| {
                                        let mut fd = form_data();
                                        fd.agree_to_terms = evt.checked();
                                        form_data.set(fd);
                                    },
                                    class: "w-4 h-4"
                                }
                                Label {
                                    label_for: "agreeToTerms",
                                    class: "text-sm",
                                    "I agree to the "
                                    a {
                                        href: "/terms",
                                        class: "text-black hover:underline",
                                        "Terms of Service"
                                    }
                                    " and "
                                    a {
                                        href: "/privacy",
                                        class: "text-black hover:underline",
                                        "Privacy Policy"
                                    }
                                }
                            }

                            // Submit Button
                            Button {
                                button_type: "submit",
                                size: ButtonSize::Lg,
                                variant: ButtonVariant::Default,
                                class: "w-full",
                                disabled: is_loading(),
                                if is_loading() { "Creating Account..." } else { "Create Account" }
                            }
                        }

                        div {
                            class: "mt-6 text-center text-sm",
                            span {
                                class: "text-gray-600",
                                "Already have an account? "
                            }
                            a {
                                href: "/login",
                                class: "font-medium text-black hover:underline",
                                "Sign in"
                            }
                        }
                    }
                }
            }
        }
    }
}