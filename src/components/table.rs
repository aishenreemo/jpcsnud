use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TableProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Table(props: TableProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        div {
            class: "relative w-full overflow-x-auto",
            table {
                class: "w-full caption-bottom text-sm {class}",
                {props.children}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableHeaderProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableHeader(props: TableHeaderProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        thead {
            class: "[&_tr]:border-b {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableBodyProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableBody(props: TableBodyProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        tbody {
            class: "[&_tr:last-child]:border-0 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableFooterProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableFooter(props: TableFooterProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        tfoot {
            class: "bg-muted/50 border-t font-medium [&>tr]:last:border-b-0 {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableRowProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableRow(props: TableRowProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        tr {
            class: "hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableHeadProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableHead(props: TableHeadProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        th {
            class: "text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableCellProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableCell(props: TableCellProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        td {
            class: "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px] {class}",
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TableCaptionProps {
    class: Option<String>,
    children: Element,
}

#[component]
pub fn TableCaption(props: TableCaptionProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    rsx! {
        caption {
            class: "text-muted-foreground mt-4 text-sm {class}",
            {props.children}
        }
    }
}
