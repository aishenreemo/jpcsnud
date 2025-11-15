use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "flex flex-col gap-2 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsListProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TabsList(props: TabsListProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            role: "tablist",
            class: "bg-muted text-muted-foreground inline-flex h-9 w-fit items-center justify-center rounded-lg p-[3px] {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsTriggerProps {
    class: Option<String>,
    children: Element,
    #[props(default)]
    selected: bool,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn TabsTrigger(props: TabsTriggerProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let active_class = if props.selected {
        "bg-background dark:text-foreground shadow-sm border-input dark:bg-input/30"
    } else {
        "text-foreground dark:text-muted-foreground"
    };
    let disabled_class = if props.disabled {
        "pointer-events-none opacity-50"
    } else {
        ""
    };

    rsx! {
        button {
            role: "tab",
            class: "inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap transition-[color,box-shadow] focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring focus-visible:ring-[3px] focus-visible:outline-1 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 {active_class} {disabled_class} {class}",
            disabled: props.disabled,
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TabsContentProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TabsContent(props: TabsContentProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            role: "tabpanel",
            class: "flex-1 outline-none {class}",
            {props.children}
        }
    }
}
