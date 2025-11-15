use crate::components::avatar::Avatar;
use crate::components::avatar::AvatarFallback;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::sheet::Sheet;
use crate::components::sheet::SheetContent;
use crate::components::sheet::SheetTrigger;
use crate::contexts::use_auth;
use crate::Route;
use dioxus::prelude::*;
use lucide_rust::dioxus::calendar_icon::Calendar;
use lucide_rust::dioxus::chart_bar_icon::ChartBar;
use lucide_rust::dioxus::credit_card_icon::CreditCard;
use lucide_rust::dioxus::menu_icon::Menu;
use lucide_rust::dioxus::settings_icon::Settings;
use lucide_rust::dioxus::user_check_icon::UserCheck;
use lucide_rust::dioxus::user_icon::User;

#[component]
pub fn Navbar() -> Element {
    let auth = use_auth();
    let mut mobile_menu_open = use_signal(|| false);

    let user = (auth.user)();
    let is_authenticated = user.is_some();
    let role = user.as_ref().map(|u| u.role.as_str()).unwrap_or("");

    let get_initials = |first: &str, last: &str| {
        format!(
            "{}{}",
            first.chars().next().unwrap_or_default(),
            last.chars().next().unwrap_or_default()
        )
        .to_uppercase()
    };

    let desktop_links = match role {
        "member" => rsx! {
            Link {
                to: Route::Home {},
                class: "text-gray-600 hover:text-black transition-colors",
                "Dashboard"
            }
            Link {
                to: Route::Home {},
                class: "text-gray-600 hover:text-black transition-colors",
                "My Events"
            }
        },
        "organizer" => rsx! {
            Link {
                to: Route::Home {},
                class: "text-gray-600 hover:text-black transition-colors",
                "Dashboard"
            }
            Link {
                to: Route::Home {},
                class: "text-gray-600 hover:text-black transition-colors",
                "Manage Events"
            }
            Link {
                to: Route::Home {},
                class: "text-gray-600 hover:text-black transition-colors",
                "Members"
            }
        },
        _ => rsx! {},
    };

    let mobile_links = match role {
        "member" => rsx! {
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                User {},
                span { "Dashboard" }
            }
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                Calendar {},
                span { "My Events" }
            }
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                Settings {},
                span { "Profile" }
            }
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                CreditCard {},
                span { "RFID Card" }
            }
        },
        "organizer" => rsx! {
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                ChartBar {},
                span { "Dashboard" }
            }
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                Calendar {},
                span { "Events" }
            }
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                UserCheck {},
                span { "Members" }
            }
            Link {
                to: Route::Home {},
                class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2",
                CreditCard {},
                span { "RFID Orders" }
            }
        },
        _ => rsx! {},
    };

    rsx! {
        header {
            class: "sticky top-0 z-50 w-full border-b bg-white/95 backdrop-blur supports-[backdrop-filter]:bg-white/60 flex h-14 items-center justify-between px-3 sm:px-4 md:px-6",
            div {
                class: "flex items-center space-x-4 md:space-x-6",
                Link {
                    to: Route::Home {},
                    class: "flex items-center space-x-2",
                    img {
                        src: "https://usdozf7pplhxfvrl.public.blob.vercel-storage.com/70006d15-e462-4267-8e7f-490ea6c53ac0-OaEBkCjJ6ahqmw6YJ7szKrdmd6pOq7",
                        alt: "JPCS NUD Logo",
                        class: "h-7 w-7 sm:h-8 sm:w-8 object-contain",
                    }
                    span {
                        class: "text-base sm:text-md font-semibold whitespace-nowrap",
                        "JPCS NUD Portal"
                    }
                }
                nav {
                    class: "hidden md:flex items-center space-x-4 text-sm",
                    Link {
                        to: Route::Home {},
                        class: "text-gray-600 hover:text-black transition-colors",
                        "Events"
                    }
                    {desktop_links}
                }
            }

            div {
                class: "flex items-center space-x-2 sm:space-x-4",
                if is_authenticated {
                    Sheet {
                        open: mobile_menu_open(),
                        on_open_change: move |open| mobile_menu_open.set(open),
                        SheetTrigger {
                            class: "md:hidden",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                class: "p-2",
                                Menu { class: "h-5 w-5" }
                                span { class: "sr-only", "Toggle menu" }
                            }
                        }
                        SheetContent {
                            side: "left",
                            class: "w-64",
                            div {
                                class: "flex flex-col space-y-4 mt-8",
                                Link { to: Route::Home {}, class: "flex items-center space-x-2 text-gray-600 hover:text-black transition-colors p-2", Calendar { class: "h-4 w-4" }, span { "Events" } }
                                {mobile_links}
                            }
                        }
                    }

                    if let Some(user) = user {
                        Button {
                            variant: ButtonVariant::Ghost,
                            class: "relative h-8 w-8 rounded-full",
                            Avatar {
                                class: "h-8 w-8",
                                AvatarFallback {
                                    class: "bg-gray-100",
                                    {get_initials(&user.first_name, &user.last_name)}
                                }
                            }
                        }
                    }
                } else {
                    div {
                        class: "flex items-center space-x-2",
                        Button { size: ButtonSize::Sm, Link { to: Route::Home {}, "Sign In" } }
                        Button { size: ButtonSize::Sm, Link { to: Route::Home {}, "Sign Up" } }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
