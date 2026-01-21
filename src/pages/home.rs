use dioxus::prelude::*;

use crate::contexts::use_auth;
use crate::components::badge::Badge;
use crate::components::badge::BadgeVariant;
use crate::components::button::Button;
use crate::components::button::ButtonVariant;
use crate::components::button::ButtonSize;
use crate::Route;

use lucide_rust::dioxus::arrow_right_icon::ArrowRight;
use lucide_rust::dioxus::credit_card_icon::CreditCard;
use lucide_rust::dioxus::shield_icon::Shield;
use lucide_rust::dioxus::zap_icon::Zap;
use lucide_rust::dioxus::circle_check_icon::CircleCheck;

#[component]
pub fn Home() -> Element {
    let auth = use_auth();

    rsx! {
        div {
            class: "min-h-screen",
            HeroSection { is_authenticated: (auth.user)().is_some() }
            BenefitsSection {}
            if !(auth.user)().is_some() {
                CtaSection {}
            }
        }
    }
}

#[component]
fn HeroSection(is_authenticated: bool) -> Element {
    rsx! {
        section {
            class: "relative py-12 sm:py-16 md:py-20 px-4 md:px-6 bg-gradient-to-br from-teal-900 via-teal-800 to-orange-900",
            div {
                class: "container max-w-4xl mx-auto text-center",
                div {
                    class: "mb-6",
                    Badge {
                        variant: BadgeVariant::Secondary,
                        class: "bg-orange-500/20 text-orange-200 border-orange-400/30",
                        "Smart RFID Technology"
                    }
                }
                h1 {
                    class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-6 tracking-tight",
                    "RFID-Powered ",
                    span {
                        class: "block text-transparent bg-clip-text bg-gradient-to-r from-orange-400 to-teal-400",
                        "Event Experience"
                    }
                }
                p {
                    class: "text-lg sm:text-xl text-gray-300 mb-8 max-w-2xl mx-auto leading-relaxed",
                    "Seamless event management with RFID technology. Join events instantly, 
            network effortlessly, and track your journey with smart card integration."
                }
                div {
                    class: "flex flex-col sm:flex-row gap-4 justify-center mb-12",
                    if !is_authenticated {
                        Button {
                            size: ButtonSize::Lg,
                            variant: ButtonVariant::Default,
                            class: "bg-white !text-black",
                            Link {
                                to: Route::Home {},
                                class: "flex flex-nowrap",
                                "Get Started ",
                                ArrowRight {
                                    class: "ml-2 h-4 w-4"
                                }
                            }
                        }
                        Button {
                            size: ButtonSize::Lg,
                            variant: ButtonVariant::Default,
                            class: "border-white text-white hover:bg-white hover:text-black",
                            Link {
                                to: Route::Home {},
                                "Browse Events"
                            }
                        }
                    } else {
                        Button {
                            size: ButtonSize::Lg,
                            variant: ButtonVariant::Outline,
                            class: "bg-white text-black hover:bg-gray-100",
                            Link {
                                to: Route::Home {},
                                "Go to Dashboard ",
                                ArrowRight {
                                    class: "ml-2 h-4 w-4"
                                }
                            }
                        }
                    }
                }
            }

            FeatureHighlights {}
        }
    }
}

#[component]
fn FeatureHighlights() -> Element {
    rsx! {
        div {
            class: "max-w-4xl mx-auto",
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 sm:gap-6 text-left pb-12 sm:pb-16 md:pb-20",
                FeatureCard {
                    title: "Smart RFID Cards",
                    description: "Instant event check-ins",
                    icon_class: "bg-orange-500/20",
                    icon_color: "text-orange-400",
                    icon: rsx! {
                        CreditCard {
                            class: "h-6 w-6 text-orange-400"
                        }
                    }
                }
                FeatureCard {
                    title: "Secure Access",
                    description: "Member-only events",
                    icon_class: "bg-teal-500/20",
                    icon_color: "text-teal-400",
                    icon: rsx! {
                        Shield {
                            class: "h-6 w-6 text-teal-400"
                        }
                    }
                }
                FeatureCard {
                    title: "Real-time Updates",
                    description: "Live event tracking",
                    icon_class: "bg-orange-600/20",
                    icon_color: "text-orange-300",
                    icon: rsx! {
                        Zap {
                            class: "h-6 w-6 text-orange-300"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FeatureCard(title: String, description: String, icon_class: String, icon_color: String, icon: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-3 p-3 sm:p-4 rounded-lg bg-white/5 border border-white/10",
            div {
                class: format!("p-2 rounded-lg {}", icon_class),
                {icon}
            }
            div {
                h3 {
                    class: "font-semibold text-white",
                    "{title}"
                }
                p {
                    class: "text-sm text-gray-400",
                    "{description}"
                }
            }
        }
    }
}

#[component]
fn BenefitsSection() -> Element {
    rsx! {
        section {
            class: "py-12 sm:py-16 px-4 md:px-6 bg-gray-50",
            div {
                class: "container max-w-4xl mx-auto",
                div {
                    class: "text-center mb-12",
                    h2 {
                        class: "text-3xl md:text-4xl font-bold text-black mb-4",
                        "Why Join Our Community?"
                    }
                    p {
                        class: "text-gray-600",
                        "Experience the future of event management with our innovative RFID system."
                    }
                }
                div {
                    class: "grid grid-cols-1 md:grid-cols-2 gap-6 sm:gap-8",
                    BenefitColumn {
                        benefits: vec![
                            ("Instant Event Access", "Skip the queues with RFID-enabled check-ins. Just tap your card and you're in."),
                            ("Exclusive Member Events", "Access premium workshops and networking events reserved for members only."),
                            ("Smart Networking", "Connect with other attendees and track your networking progress."),
                        ]
                    }
                    BenefitColumn {
                        benefits: vec![
                            ("Digital Membership", "Manage your profile, track event history, and update preferences online."),
                            ("Real-time Updates", "Get instant notifications about event changes and new opportunities."),
                            ("Secure & Reliable", "Advanced security ensures your data and access credentials are protected."),
                        ]
                    }
                }
            }
        }
    }
}

#[component]
fn BenefitColumn(benefits: Vec<(&'static str, &'static str)>) -> Element {
    rsx! {
        div {
            class: "space-y-6",
            for (title, description) in benefits {
                BenefitItem {
                    title: title.to_string(),
                    description: description.to_string()
                }
            }
        }
    }
}

#[component]
fn BenefitItem(title: String, description: String) -> Element {
    rsx! {
        div {
            class: "flex items-start space-x-4",
            CircleCheck {
                class: "h-6 w-6 text-green-500 mt-0.5 flex-shrink-0"
            }
            div {
                h3 {
                    class: "font-semibold text-black mb-2",
                    "{title}"
                }
                p {
                    class: "text-gray-600",
                    "{description}"
                }
            }
        }
    }
}

#[component]
fn CtaSection() -> Element {
    rsx! {
        section {
            class: "py-12 sm:py-16 px-4 md:px-6 bg-black",
            div {
                class: "container max-w-4xl mx-auto text-center",
                h2 {
                    class: "text-3xl md:text-4xl font-bold text-white mb-4",
                    "Ready to Get Started?"
                }
                p {
                    class: "text-gray-300 mb-8 max-w-2xl mx-auto",
                    "Join thousands of members who are already enjoying seamless event experiences."
                }
                div {
                    class: "flex flex-col sm:flex-row gap-4 justify-center",
                    Button {
                        size: ButtonSize::Lg,
                        variant: ButtonVariant::Default,
                        class: "bg-white !text-black hover:bg-black/10 !hover:text-white",
                        Link {
                            to: Route::Home {},
                            "Create Account"
                        }
                    }
                    Button {
                        size: ButtonSize::Lg,
                        variant: ButtonVariant::Outline,
                        class: "hover:bg-black/10 hover:text-white border-white",
                        Link {
                            to: Route::Home {},
                            "Sign In"
                        }
                    }
                }
            }
        }
    }
}
