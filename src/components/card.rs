use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardHeaderProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6 has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardTitleProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardTitle(props: CardTitleProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "leading-none font-semibold {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardDescriptionProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardDescription(props: CardDescriptionProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "text-muted-foreground text-sm {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardActionProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardAction(props: CardActionProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "col-start-2 row-span-2 row-start-1 self-start justify-self-end {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardContentProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardContent(props: CardContentProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "px-6 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardFooterProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    let class = props.class.unwrap_or_default();
    rsx! {
        div {
            class: "flex items-center px-6 [.border-t]:pt-6 {class}",
            {props.children}
        }
    }
}
