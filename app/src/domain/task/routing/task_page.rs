use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::layout::message_banner::{Messages, show_info};
use crate::components::ui::button::Button;
use crate::components::ui::button_link::ButtonLink;
use crate::domain::task::model::task::Task;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::{DeleteTask, get_task};

#[component]
pub fn TaskPage() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let task_resource = Resource::new_blocking(id, async move |id| get_task(id.parse().unwrap_or(0)).await);

    view! {
        <div class="container p-4">
            <div class="message is-dark">
                <div class="message-header">
                    <p>{"Сделать"}</p>
                </div>

                <div class="message-body">
                    <Transition
                        fallback=move || view! { <TaskDetails task=Task {title: Some("...".to_owned()), description: Some("...".to_owned()), ..Task::default()} /> }
                        >
                        {move || task_resource.get().map(|data| {
                            let task = data.ok().unwrap_or_default();
                            view! {
                                <TaskDetails task />
                            }
                        })}
                    </Transition>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TaskDetails(task: Task) -> impl IntoView {
    let delete_task = ServerAction::<DeleteTask>::new();

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    Effect::new(move |_| {
        if let Some(Ok(_)) = delete_task.value().get() {
            show_info("Задача удалена!".to_owned(), messages);
            delete_task.clear();
        }
    });

    view! {
        <div class="media">
            <div class="media-left">
                {
                    if task.completed_at.is_some() {
                        view! {<span class="is-size-3">{"✅"}</span>}.into_any()
                    } else {
                        view! {<span class="is-size-3">{"❌"}</span>}.into_any()
                    }
                }
            </div>
            <div>
                <p class="title is-size-4 is-size-6-mobile">
                    {task.title.to_owned()}
                </p>
                <p class="subtitle is-6">
                    {task.priority_name()}
                </p>
            </div>
        </div>

        <div class="content">
            <p>{task.description.to_owned()}</p>
        </div>

        <div class="buttons">
            <ButtonLink
                class_name="is-primary is-size-7-mobile".to_owned()
                href={TaskRoutes::edit_url(task.id.unwrap_or_default())}
                label="Изменить".to_owned()
            />

            <ActionForm action=delete_task>
                <input type="hidden" name="id" value={task.id.unwrap_or_default()} />
                <Button
                    class_name="is-danger is-light is-size-7-mobile".to_owned()
                    label="Удалить".to_owned()
                    loading=move || delete_task.pending().get()
                    disabled=move || task.id.is_none() || delete_task.pending().get()
                    on_click=move |_| {}
                />
            </ActionForm>
        </div>
    }
}
