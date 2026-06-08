use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectInput(
    name: String,
    #[prop(optional)] value: String,
    #[prop(optional)] class_name: String,
    label: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>
) -> impl IntoView {
    view! {
        <div class=format!("select {}", class_name)>
            <select aria-label={label}
                id = {name.to_owned()}
                name = {name}
                prop:value = {value.to_owned()}
                on:change=move |ev| {
                    let val = event_target_value(&ev);
                    on_change.run(val)
                }
            >
                <option value={""}>{not_selected_text}</option>

                {
                    options.into_iter()
                    .map(|option| view! { 
                        <option value={option.0.to_owned()} selected={option.0 == Some(value.to_owned())}>{option.1}</option>
                    }).collect::<Vec<_>>()
                }

            </select>
        </div>
    }
}
