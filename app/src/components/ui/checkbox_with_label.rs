use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn CheckboxWithLabel(
    name: String,
    label: String,
    value: bool,
    #[prop(optional)] class_name: String,
) -> impl IntoView {
    view! {
        <div class="control">
            <label class=format!("b-checkbox checkbox {}", class_name)>
                <input type="checkbox" name=name checked=value />
                <span class="check is-warning"></span>
                <span class="control-label">{label}</span>
            </label>
        </div>
    }
}
