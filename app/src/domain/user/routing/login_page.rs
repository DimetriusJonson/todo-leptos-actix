use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::hooks::use_query_map;
use validator::Validate;

use crate::common::validate_helper::{
    ui_build_common_error, ui_build_validation_errors, validate_form, validation_errors_to_map,
};
use crate::components::layout::message_banner::{Messages, show_info, show_server_error};
use crate::components::ui::button::Button;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::user::model::login_params::LoginParams;
use crate::domain::user::user_services::Login;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login = ServerAction::<Login>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let query_map = use_query_map();
    let def_user_name = move || query_map.with(|m| m.get("defUserName"));

    let (errors, set_validation_errors) = signal(HashMap::<String, Vec<String>>::new());

    let validation_errors: Signal<HashMap<String, Vec<String>>> = Signal::derive(move || {
        let mut result = errors.get();
        result.extend(login.value().with(ui_build_validation_errors));
        result
    });
    let common_error = move || ui_build_common_error(validation_errors);

    Effect::new(move |_| match login.value().get() {
        Some(res) => match res {
            Ok(_) => {
                show_info("Вы вошли!".to_owned(), messages);
                login.clear();
            }
            Err(err) => show_server_error(err, messages),
        },
        None => (),
    });

    view! {
        <div class="container p-4">
            <MainTitle title=|| "Вход в систему".to_owned() />

            <ActionForm action=login
                on:submit:capture=move |event| {
                    if let Ok(params) = Login::from_event(&event) {
                        if let Err(validation_errors) = params.validate() {
                            set_validation_errors.set(validation_errors_to_map(validation_errors));
                            event.prevent_default();
                        }
                    } else {
                        event.prevent_default();
                    }
                }
                on:input=move |event| {
                        validate_form(event, set_validation_errors, LoginParams::default());
                        login.clear();
                    }
            >
                <input name="params[version]" type="hidden" value={move || login.version().get()} />

                <div class="help is-danger is-size-5 py-4">{common_error}</div>

                <fieldset disabled=login.pending()>
                    <div class="field">
                        { move || view! {
                                <TextWithError input_type="text".to_owned() name="params[name]".to_owned()
                                    placeholder="Имя пользователя".to_owned()
                                    validation_errors
                                    value={def_user_name().unwrap_or_default()}
                                />
                            }
                        }
                    </div>

                    <div class="field">
                        <TextWithError input_type="password".to_owned() name="params[password]".to_owned()
                            placeholder="Пароль".to_owned()
                            validation_errors
                        />
                    </div>

                    <div class="field">
                        <div class="control">
                            <Button
                                class_name="is-primary".to_owned()
                                label="Войти".to_owned()
                                loading=move || login.pending().get()
                                on_click=move |_| {}
                                disabled=move || login.pending().get()
                            />
                        </div>
                    </div>
                </fieldset>
            </ActionForm>
        </div>
    }
}
