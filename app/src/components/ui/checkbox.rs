use leptos::prelude::*;

#[component]
pub fn Checkbox(
    name: String,
    title: String,
    label: String,
    value: bool,
    #[prop(optional)] class_name: String,
    disabled: impl Fn() -> bool + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <label class=format!("b-checkbox checkbox {}", class_name)>
            <input aria-label={label} type="checkbox" name=name.to_owned() checked=value disabled=disabled/>
            <span class="check is-warning" title=title></span>
        </label>
    }
}
