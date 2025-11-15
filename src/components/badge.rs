use dioxus::prelude::*;

#[allow(unused)]
#[derive(Clone, Default, PartialEq)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

#[allow(unused)]
impl BadgeVariant {
    fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground hover:bg-primary/90",
            BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground hover:bg-secondary/90",
            BadgeVariant::Destructive => "border-transparent bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
            BadgeVariant::Outline => "text-foreground hover:bg-accent hover:text-accent-foreground",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    variant: BadgeVariant,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let base = "inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0 gap-1 transition-[color,box-shadow] overflow-hidden";
    let variant_class = props.variant.class();
    let class = props.class.unwrap_or_default();
    rsx! {
        span {
            class: "{base} {variant_class} {class}",
            {props.children}
        }
    }
}
