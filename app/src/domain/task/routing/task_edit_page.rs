use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use validator::Validate;

use crate::common::validate_helper::{
    ui_build_common_error, ui_build_validation_errors, validate_form, validation_errors_to_map,
};
use crate::components::layout::message_banner::{Messages, show_info, show_server_error};
use crate::components::ui::button::Button;
use crate::components::ui::button_link::ButtonLink;
use crate::components::ui::checkbox_with_label::CheckboxWithLabel;
use crate::components::ui::main_title::MainTitle;
use crate::components::ui::select_with_error::SelectWithError;
use crate::components::ui::text_area::TextArea;
use crate::components::ui::text_with_error::TextWithError;
use crate::domain::home::routing::routes::HomeRoutes;
use crate::domain::task::model::task::Task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{UpdateOrCreateTask, get_priorities, get_task};

#[component]
pub fn TaskEditPage() -> impl IntoView {
    let params = use_params_map();
    let navigate = leptos_router::hooks::use_navigate();
    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let task_resource = Resource::new_blocking(
        move || params.read().get("id"),
        async move |id| get_task(id.unwrap_or_default().parse().unwrap_or(0)).await,
    );
    let priorities_resource = OnceResource::new(get_priorities());

    Effect::new(move |_| {
        if let Some(Err(err)) = task_resource.get() {
            show_server_error(err, messages);
            navigate("/", Default::default());
        }
    });

    view! {
        <div class="container p-4">
            <MainTitle title=move || match params.read().get("id") {
        Some(_) => "Редактировать задачу".to_owned(),
        None => "Создать задачу".to_owned(),
    } />
            <Transition fallback=move || view! { <TaskEditForm task={Task::default()} priorities={None} disabled=true /> }>
                {move || Suspend::new(async move {
                    let task = task_resource.await.unwrap_or_default();
                    let priorities = priorities_resource.await.ok();
                    view! {
                        <TaskEditForm task priorities disabled=false />
                    }
                })}
            </Transition>
        </div>
    }
}

#[component]
pub fn TaskEditForm(
    task: Task,
    priorities: Option<Vec<(Option<String>, String)>>,
    disabled: bool,
) -> impl IntoView {
    let update_or_create_task = ServerAction::<UpdateOrCreateTask>::new();

    let (errors, set_validation_errors) = signal(HashMap::<String, Vec<String>>::new());

    let validation_errors: Signal<HashMap<String, Vec<String>>> = Signal::derive(move || {
        let mut result = errors.get();
        result.extend(update_or_create_task.value().with(ui_build_validation_errors));
        result
    });
    let common_error = move || ui_build_common_error(validation_errors);

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    Effect::new(move |_| match update_or_create_task.value().get() {
        Some(res) => match res {
            Ok(_) => {
                show_info("Задача сохранена!".to_owned(), messages);
                update_or_create_task.clear();
            }
            Err(err) => show_server_error(err, messages),
        },
        None => (),
    });

    view! {
        <ActionForm action=update_or_create_task
            on:submit:capture=move |event| {
                if let Ok(params) = UpdateOrCreateTask::from_event(&event) {
                    if let Err(validation_errors) = params.validate() {
                        set_validation_errors.set(validation_errors_to_map(validation_errors));
                        event.prevent_default();
                    }
                } else {
                    event.prevent_default();
                }
            }
            on:input=move |event| {
                    validate_form(event, set_validation_errors, Task::default());
                    update_or_create_task.clear();
                }
        >
            <input type="hidden" name="task[id]" value=task.id />

            <div class="help is-danger is-size-5 py-4">{common_error}</div>

            <fieldset disabled={move || { disabled || update_or_create_task.pending().get()}}>
                <div class="level">
                    <div class="level-left">
                        <div class="level-item">
                            <SelectWithError
                                name="task[priority]".to_owned()
                                label="Приоритет:".to_owned()
                                error_class_name="pl-4".to_owned()
                                validation_errors
                                options=priorities.unwrap_or_default()
                                not_selected_text="Не выбран".to_owned()
                                value=task.priority.unwrap_or_default()
                                on_change=|_| {}
                            />
                        </div>
                    </div>

                    <div class="level-right">
                        <div class="level-item">
                            <CheckboxWithLabel
                                name="task[completed_at]".to_owned()
                                value=task.completed_at.is_some()
                                label="Завершена".to_owned()
                            />
                        </div>
                    </div>

                </div>
                <div class="field">
                    <TextWithError
                        input_type="text".to_owned()
                        name="task[title]".to_owned()
                        placeholder="Название".to_owned()
                        validation_errors
                        value=task.title.unwrap_or_default()
                    />
                </div>

                <div class="field">
                    <TextArea
                        name="task[description]".to_owned()
                        placeholder="Описание".to_owned()
                        value=task.description.unwrap_or_default()
                        on_change=|_| {}
                    />
                </div>

                <div class="field is-grouped">
                    <div class="control">

                        <Button
                            class_name="is-primary".to_owned()
                            label="Сохранить".to_owned()
                            loading=move || update_or_create_task.pending().get()
                            on_click=move |_| {}
                            disabled=move || update_or_create_task.pending().get()
                        />
                    </div>
                    <div class="control">
                        <ButtonLink
                            class_name="is-light".to_owned()
                            label="Отмена".to_owned()
                            href=match task.id {
                                Some(id) => TaskRoutes::details_url(id),
                                None => HomeRoutes::base_url().to_owned(),
                            }.to_owned()
                        />
                    </div>
                </div>

            </fieldset>
        </ActionForm>


    }
}
