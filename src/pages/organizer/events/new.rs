use dioxus::prelude::*;
use crate::contexts::use_auth;
use crate::components::button::Button;
use crate::components::button::ButtonSize;
use crate::components::button::ButtonVariant;
use crate::components::input::Input;
use crate::components::label::Label;
use crate::components::textarea::Textarea;
use crate::components::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use crate::components::badge::Badge;
use crate::components::page_header::PageHeader;
use crate::components::checkbox::Checkbox;
use crate::components::radio_group::RadioGroup;
use crate::components::separator::Separator;

use lucide_rust::dioxus::{
    calendar_icon::Calendar,
    map_pin_icon::MapPin,
    users_icon::Users,
    dollar_sign_icon::DollarSign,
    globe_icon::Globe,
    lock_icon::Lock,
    clock_icon::Clock,
    save_icon::Save,
    eye_icon::Eye,
};

#[component]
pub fn CreateEventPage() -> Element {
    let auth = use_auth();
    let mut is_saving = use_signal(|| false);
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut date = use_signal(String::new);
    let mut end_date = use_signal(String::new);
    let mut location = use_signal(String::new);
    let mut max_attendees = use_signal(String::new);
    let mut is_public = use_signal(|| true);
    let mut requires_registration = use_signal(|| true);
    let mut registration_deadline = use_signal(String::new);
    let mut price = use_signal(String::new);
    let mut tags = use_signal(String::new);
    let mut image = use_signal(String::new);

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

    let handle_submit = move |status: &str| {
        if title().is_empty() || description().is_empty() || date().is_empty() || location().is_empty() {
            // Would show toast error in real implementation
            return;
        }

        is_saving.set(true);
        spawn(async move {
            async_std::task::sleep(std::time::Duration::from_secs(2)).await;
            is_saving.set(false);
            // Would navigate and show toast in real implementation
        });
    };

    let is_form_valid = !title().is_empty() && !description().is_empty() && !date().is_empty() && !location().is_empty();

    rsx! {
        div {
            class: "container mx-auto py-8 px-4 md:px-6 max-w-4xl",
            PageHeader {
                title: "Create New Event",
                description: "Set up your event details and publish when ready"
            }

            form {
                class: "space-y-8",

                // Basic Information
                Card {
                    CardHeader {
                        CardTitle { "Basic Information" }
                        CardDescription { "Essential details about your event" }
                    }
                    CardContent {
                        class: "space-y-6",
                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "title",
                                "Event Title *"
                            }
                            Input {
                                id: "title",
                                placeholder: "Enter event title",
                                value: title(),
                                onchange: move |val: String| {
                                    title.set(val);
                                },
                            }
                        }

                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "description",
                                "Event Description *"
                            }
                            Textarea {
                                id: "description",
                                placeholder: "Describe your event, what attendees can expect, and any important details...",
                                value: description(),
                                onchange: move |val: String| {
                                    description.set(val);
                                },
                            }
                        }

                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "tags",
                                "Tags"
                            }
                            Input {
                                id: "tags",
                                placeholder: "workshop, networking, tech, startup (comma separated)",
                                value: tags(),
                                onchange: move |val: String| {
                                    tags.set(val);
                                },
                            }
                            p {
                                class: "text-sm text-gray-500",
                                "Add tags to help members find your event. Separate multiple tags with commas."
                            }
                        }

                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "image",
                                "Featured Image URL"
                            }
                            Input {
                                id: "image",
                                r#type: "url",
                                placeholder: "https://example.com/image.jpg",
                                value: image(),
                                onchange: move |val: String| {
                                    image.set(val);
                                },
                            }
                            p {
                                class: "text-sm text-gray-500",
                                "Optional: Add a URL to an image that represents your event (recommended: 800x400px)"
                            }
                        }
                    }
                }

                // Date & Time
                Card {
                    CardHeader {
                        CardTitle {
                            class: "flex items-center",
                            Calendar {
                                class: "mr-2 h-5 w-5"
                            }
                            "Date & Time"
                        }
                        CardDescription {
                            "When will your event take place?"
                        }
                    }
                    CardContent {
                        class: "space-y-4",
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            div {
                                class: "space-y-2",
                                Label {
                                    r#for: "date",
                                    "Start Date & Time *"
                                }
                                Input {
                                    id: "date",
                                    r#type: "datetime-local",
                                    value: date(),
                                    onchange: move |val: String| {
                                        date.set(val);
                                    },
                                }
                            }

                            div {
                                class: "space-y-2",
                                Label {
                                    r#for: "endDate",
                                    "End Date & Time"
                                }
                                Input {
                                    id: "endDate",
                                    r#type: "datetime-local",
                                    value: end_date(),
                                    onchange: move |val: String| {
                                        end_date.set(val);
                                    },
                                }
                                p {
                                    class: "text-xs text-gray-500",
                                    "Optional for single-day events"
                                }
                            }
                        }

                        if requires_registration() {
                            div {
                                class: "space-y-2",
                                Label {
                                    r#for: "registrationDeadline",
                                    "Registration Deadline"
                                }
                                Input {
                                    id: "registrationDeadline",
                                    r#type: "datetime-local",
                                    value: registration_deadline(),
                                    onchange: move |val: String| {
                                        registration_deadline.set(val);
                                    },
                                }
                                p {
                                    class: "text-sm text-gray-500",
                                    "When should registration close? Leave blank to allow registration until event starts."
                                }
                            }
                        }
                    }
                }

                // Location & Capacity
                Card {
                    CardHeader {
                        CardTitle {
                            class: "flex items-center",
                            MapPin {
                                class: "mr-2 h-5 w-5"
                            }
                            "Location & Capacity"
                        }
                        CardDescription {
                            "Where will your event be held?"
                        }
                    }
                    CardContent {
                        class: "space-y-4",
                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "location",
                                "Event Location *"
                            }
                            Input {
                                id: "location",
                                placeholder: "e.g., Community Center, 123 Main St, San Francisco, CA",
                                value: location(),
                                onchange: move |val: String| {
                                    location.set(val);
                                },
                            }
                        }

                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "maxAttendees",
                                "Maximum Attendees"
                            }
                            Input {
                                id: "maxAttendees",
                                r#type: "number",
                                placeholder: "e.g., 100",
                                value: max_attendees(),
                                onchange: move |val: String| {
                                    max_attendees.set(val);
                                },
                            }
                            p {
                                class: "text-sm text-gray-500",
                                "Leave blank for unlimited capacity"
                            }
                        }
                    }
                }

                // Access & Registration
                Card {
                    CardHeader {
                        CardTitle {
                            class: "flex items-center",
                            Users {
                                class: "mr-2 h-5 w-5"
                            }
                            "Access & Registration"
                        }
                        CardDescription {
                            "Who can attend and how do they register?"
                        }
                    }
                    CardContent {
                        class: "space-y-6",
                        div {
                            class: "space-y-4",
                            div {
                                label {
                                    class: "text-base font-medium",
                                    "Event Visibility"
                                }
                                div {
                                    class: "mt-2 space-y-2",
                                    div {
                                        class: "flex items-center space-x-2 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer",
                                        onclick: move |_| {
                                            is_public.set(true);
                                        },
                                        input {
                                            r#type: "radio",
                                            name: "visibility",
                                            value: "public",
                                            checked: is_public(),
                                        }
                                        Globe {
                                            class: "h-5 w-5 text-blue-500"
                                        }
                                        div {
                                            div {
                                                class: "font-medium",
                                                "Public Event"
                                            }
                                            div {
                                                class: "text-sm text-gray-500",
                                                "Anyone can view and register for this event"
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex items-center space-x-2 p-3 border rounded-lg hover:bg-gray-50 cursor-pointer",
                                        onclick: move |_| {
                                            is_public.set(false);
                                        },
                                        input {
                                            r#type: "radio",
                                            name: "visibility",
                                            value: "private",
                                            checked: !is_public(),
                                        }
                                        Lock {
                                            class: "h-5 w-5 text-purple-500"
                                        }
                                        div {
                                            div {
                                                class: "font-medium",
                                                "Members Only"
                                            }
                                            div {
                                                class: "text-sm text-gray-500",
                                                "Only registered members can view and attend"
                                            }
                                        }
                                    }
                                }
                            }

                            div {
                                class: "flex items-center space-x-2",
                                Checkbox {
                                    id: "requiresRegistration",
                                    checked: requires_registration(),
                                    onchange: move |checked: bool| {
                                        requires_registration.set(checked);
                                    },
                                }
                                Label {
                                    r#for: "requiresRegistration",
                                    "Require registration to attend"
                                }
                            }
                        }
                    }
                }

                // Pricing
                Card {
                    CardHeader {
                        CardTitle {
                            class: "flex items-center",
                            DollarSign {
                                class: "mr-2 h-5 w-5"
                            }
                            "Event Pricing"
                        }
                        CardDescription {
                            "Set your event pricing (optional)"
                        }
                    }
                    CardContent {
                        div {
                            class: "space-y-2",
                            Label {
                                r#for: "price",
                                "Ticket Price (USD)"
                            }
                            Input {
                                r#type: "number",
                                placeholder: "0.00",
                                value: price(),
                                onchange: move Some(|val: String| {
                                    price.set(val);
                                }),
                            }
                            p {
                                class: "text-sm text-gray-500",
                                "Leave blank or set to 0 for free events"
                            }
                        }
                    }
                }

                // Preview
                Card {
                    class: "bg-blue-50 border-blue-200",
                    CardHeader {
                        CardTitle {
                            class: "flex items-center text-blue-800",
                            Eye {
                                class: "mr-2 h-5 w-5"
                            }
                            "Event Preview"
                        }
                    }
                    CardContent {
                        class: "space-y-3",
                        div {
                            h3 {
                                class: "font-semibold text-blue-900",
                                if title().is_empty() { "Event Title" } else { "{title()}" }
                            }
                            p {
                                class: "text-sm text-blue-700 mt-1",
                                if description().is_empty() { "Event description will appear here..." } else { "{description()}" }
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2 text-sm text-blue-700",
                            div {
                                class: "flex items-center",
                                Calendar {
                                    class: "mr-1 h-4 w-4"
                                }
                                if date().is_empty() { "Date TBD" } else { "{date()}" }
                            }
                            div {
                                class: "flex items-center",
                                MapPin {
                                    class: "mr-1 h-4 w-4"
                                }
                                if location().is_empty() { "Location TBD" } else { "{location()}" }
                            }
                            div {
                                class: "flex items-center",
                                if is_public() {
                                    Globe {
                                        class: "mr-1 h-4 w-4"
                                    }
                                } else {
                                    Lock {
                                        class: "mr-1 h-4 w-4"
                                    }
                                }
                                if is_public() { "Public" } else { "Members Only" }
                            }
                            if !price().is_empty() && price().parse::<f64>().unwrap_or(0.0) > 0.0 {
                                div {
                                    class: "flex items-center",
                                    DollarSign {
                                        class: "mr-1 h-4 w-4"
                                    }
                                    "{price()}"
                                }
                            }
                        }
                    }
                }

                // Form Actions
                Card {
                    CardContent {
                        class: "flex flex-col sm:flex-row gap-4 pt-6",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Lg,
                            class: "flex-1",
                            disabled: is_saving() || !is_form_valid,
                            onclick: move |_| {
                                handle_submit("draft");
                            },
                            Save {
                                class: "mr-2 h-4 w-4"
                            }
                            if is_saving() { "Saving..." } else { "Save as Draft" }
                        }
                        Button {
                            variant: ButtonVariant::Default,
                            size: ButtonSize::Lg,
                            class: "flex-1",
                            disabled: is_saving() || !is_form_valid,
                            onclick: move |_| {
                                handle_submit("published");
                            },
                            Globe {
                                class: "mr-2 h-4 w-4"
                            }
                            if is_saving() { "Publishing..." } else { "Publish Event" }
                        }
                    }
                }
            }
        }
    }
}