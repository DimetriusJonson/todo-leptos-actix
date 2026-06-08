use leptos::{ev::MouseEvent, html, prelude::*};

#[component]
pub fn Button(
    #[prop(optional)] id: i32,
    label: String,
    #[prop(optional)] class_name: String,
    loading: impl Fn() -> bool + Send + Sync + 'static,
    disabled: impl Fn() -> bool + Send + Sync + 'static,
    on_click: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let button_element: NodeRef<html::Button> = NodeRef::new();
    let aria_label = label.to_owned();

    view! {
        <button
            node_ref=button_element
            id={id}
            aria-label={aria_label}
            class=format!("button is-rounded {}", class_name)
            class:is-loading=loading
            on:click=on_click
            on:mouseup=move |_| if let Some(button) = button_element.get() { button.blur().unwrap(); }
            disabled=disabled>
            {label}
        </button>
    }
}
