use dioxus::prelude::*;

#[allow(unused)]
#[derive(Clone, Default, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[allow(unused)]
impl ButtonVariant {
    fn class(&self) -> &'static str {
        match self {
            ButtonVariant::Default => "hover:bg-primary/90 bg-primary text-primary-foreground",
            ButtonVariant::Destructive => "bg-destructive text-white hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
            ButtonVariant::Outline => "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50",
            ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
            ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
            ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        }
    }
}

#[allow(unused)]
#[derive(Clone, Default, PartialEq)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
    IconLg,
}

#[allow(unused)]
impl ButtonSize {
    fn class(&self) -> &'static str {
        match self {
            ButtonSize::Default => "h-9 px-4 py-2 has-[>svg]:px-3",
            ButtonSize::Sm => "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
            ButtonSize::Lg => "h-10 rounded-md px-6 has-[>svg]:px-4",
            ButtonSize::Icon => "size-9",
            ButtonSize::IconSm => "size-8",
            ButtonSize::IconLg => "size-10",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default)]
    variant: ButtonVariant,
    #[props(default)]
    size: ButtonSize,
    class: Option<String>,
    children: Element,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive";
    let variant_class = props.variant.class();
    let size_class = props.size.class();
    let class = props.class.unwrap_or_default();

    rsx! {
        button {
            class: "{base} {variant_class} {size_class} {class}",
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
