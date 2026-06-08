use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn TextArea(
    name: String,
    #[prop(optional)] value: String,
    placeholder: String,
    #[prop(into)] on_change: Callback<String>,
) -> impl IntoView {
    view! {
        <textarea
            class="textarea"
            rows="4"
            cols="50".to_ascii_lowercase()
            name=name
            placeholder=placeholder
            on:change=move |ev| {
                let val = event_target_value(&ev);
                on_change.run(val)
            }
        >{value}</textarea>
    }
}
