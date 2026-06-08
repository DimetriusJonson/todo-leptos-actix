use std::collections::HashMap;

use leptos::prelude::*;

use crate::common::validate_helper::{extract_form_field_name, ui_extract_field_errors};

#[component]
pub fn TextWithError(
    name: String,
    placeholder: String,
    input_type: String,
    #[prop(optional)] value: String,
    validation_errors: Signal<HashMap<String, Vec<String>>>,
) -> impl IntoView {
    view! {
        <div class="control">
            <input
                class={"input"}
                class:is-danger=move || false
                type=input_type
                id=name.to_owned()
                name=name.to_owned()
                value=value
                placeholder=placeholder
            />
        </div>

        {
            let field_name = extract_form_field_name(name.to_owned());
            move || {
                let errors = ui_extract_field_errors(&field_name, validation_errors);
                errors.map(|list| list.into_iter().map(|msg| view!{ <p class="help is-danger">{msg}</p>}).collect::<Vec<_>>().into_iter().map(|msg| view!{ <p class="help is-danger">{msg}</p>}).collect::<Vec<_>>())
            }
        }


    }
}
