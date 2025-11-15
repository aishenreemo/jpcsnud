use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PageHeaderProps {
    title: String,
    #[props(default)]
    description: Option<String>,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn PageHeader(props: PageHeaderProps) -> Element {
    let class = props.class.clone().unwrap_or_default();

    rsx! {
        div {
            class: "flex items-center justify-between pb-6 {class}",
            div {
                class: "space-y-1",
                h1 {
                    class: "text-2xl font-bold tracking-tight text-black",
                    "{props.title}"
                }
                if let Some(desc) = &props.description {
                    p {
                        class: "text-gray-600",
                        "{desc}"
                    }
                }
            }
        }
    }
}
