use dioxus::prelude::*;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

use lucide_rust::dioxus::{
    eye_icon::Eye,
    eye_off_icon::EyeOff,
};

#[derive(Clone)]
struct FormData {
    email: String,
    password: String,
}

#[component]
pub fn Login() -> Element {
    let mut form_data = use_signal(FormData {
        email: String::new(),
        password: String::new(),
    });
    let mut is_loading = use_signal(false);
    let mut show_password = use_signal(false);

    let handle_submit = move |_| {
        let fd = form_data();
        
        if fd.email.is_empty() || fd.password.is_empty() {
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
                        "Welcome back"
                    }
                    p {
                        class: "mt-2 text-sm sm:text-base text-gray-600",
                        "Sign in to your account to continue"
                    }
                }

                Card {
                    CardHeader {
                        CardTitle { "Sign In" }
                        CardDescription {
                            "Enter your credentials to access your account"
                        }
                    }
                    CardContent {
                        form {
                            class: "space-y-4",
                            onsubmit: handle_submit,

                            // Email Field
                            div {
                                class: "space-y-2",
                                Label {
                                    label_for: "email",
                                    "Email address"
                                }
                                Input {
                                    id: "email",
                                    name: "email",
                                    input_type: "email",
                                    placeholder: "Enter your email",
                                    value: form_data().email,
                                    onchange: move |evt| {
                                        let mut fd = form_data();
                                        fd.email = evt.value();
                                        form_data.set(fd);
                                    },
                                    required: true
                                }
                            }

                            // Password Field
                            div {
                                class: "space-y-2",
                                Label {
                                    label_for: "password",
                                    "Password"
                                }
                                div {
                                    class: "relative",
                                    Input {
                                        id: "password",
                                        name: "password",
                                        input_type: if show_password() { "text" } else { "password" },
                                        placeholder: "Enter your password",
                                        value: form_data().password,
                                        onchange: move |evt| {
                                            let mut fd = form_data();
                                            fd.password = evt.value();
                                            form_data.set(fd);
                                        },
                                        required: true
                                    }
                                    Button {
                                        button_type: "button",
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::Sm,
                                        class: "absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent",
                                        onclick: move |_| show_password.toggle(),
                                        if show_password() {
                                            EyeOff {
                                                class: "h-4 w-4 text-gray-500"
                                            }
                                        } else {
                                            Eye {
                                                class: "h-4 w-4 text-gray-500"
                                            }
                                        }
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
                                if is_loading() { "Signing in..." } else { "Sign In" }
                            }
                        }

                        div {
                            class: "mt-6 text-center text-sm",
                            span {
                                class: "text-gray-600",
                                "Don't have an account? "
                            }
                            a {
                                href: "/register",
                                class: "font-medium text-black hover:underline",
                                "Sign up"
                            }
                        }
                    }
                }

                // Demo Credentials Card
                Card {
                    class: "bg-blue-50 border-blue-200",
                    CardHeader {
                        CardTitle {
                            class: "text-sm text-blue-800",
                            "Demo Credentials"
                        }
                    }
                    CardContent {
                        class: "pt-0",
                        div {
                            class: "text-xs sm:text-sm text-blue-700 space-y-2",
                            div {
                                strong { "Member: " }
                                "john.doe@example.com / password123"
                            }
                            div {
                                strong { "Organizer: " }
                                "sarah.admin@example.com / password123"
                            }
                        }
                    }
                }
            }
        }
    }
}