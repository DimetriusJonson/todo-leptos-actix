use std::collections::HashMap;

use crate::{
    common::validate_helper::{extract_form_field_name, ui_extract_field_errors},
    components::ui::select_input::SelectInput,
};
use leptos::prelude::*;

pub type SelectOption = (Option<String>, String);

#[component]
pub fn SelectWithError(
    name: String,
    label: String,
    #[prop(optional)] error_class_name: String,
    #[prop(optional)] value: String,
    #[prop(optional)] not_selected_text: String,
    options: Vec<SelectOption>,
    #[prop(into)] on_change: Callback<String>,
    validation_errors: Signal<HashMap<String, Vec<String>>>,
) -> impl IntoView {
    view! {
            <label class="label mx-2" for=name.to_owned()>
                {label.to_owned()}
            </label>
            <SelectInput label={label} name={name.to_owned()} not_selected_text=not_selected_text options=options value=value on_change=on_change
            />

        {
            let field_name = extract_form_field_name(name.to_owned());
            move || {
                let errors = ui_extract_field_errors(&field_name, validation_errors);
                errors.map(|list| list.into_iter().map(|msg| view!{ <p class=format!("help is-danger {}", error_class_name)>{msg}</p>}).collect::<Vec<_>>().into_iter().map(|msg| view!{ <p class="help is-danger">{msg}</p>}).collect::<Vec<_>>())
            }
        }

    }
}
