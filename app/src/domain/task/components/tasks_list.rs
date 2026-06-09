use leptos::prelude::*;
use leptos::reactive::spawn_local;
use web_sys::{Event, HtmlInputElement};

use crate::components::layout::message_banner::{Messages, show_info, show_server_error};
use crate::components::ui::checkbox::Checkbox;
use crate::domain::task::model::task::{Task, filter_task};
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::task_services::change_completed_task;

#[component]
pub fn TasksList(filter: ReadSignal<Option<String>>) -> impl IntoView {
    let messages: Messages = use_context::<Messages>().expect("Cant get messages context!");

    let tasks_resource = use_context::<Resource<Result<Vec<Task>, ServerFnError>>>().unwrap();

    let (change_completed_in_progress, set_change_completed_in_progress) = signal(true);
    Effect::new(move |_| {
        set_change_completed_in_progress.set(false);
    });

    let completed_on_change = move |event: Event| {
        event.prevent_default();

        let checkbox = event_target::<HtmlInputElement>(&event);
        let name = checkbox.name();
        let value = checkbox.checked();
        checkbox.set_checked(!value);

        if let Some(index_und) = name.find('_')
            && let Ok(id) = name[index_und + 1..].parse::<i64>()
        {
            spawn_local(async move {
                set_change_completed_in_progress.set(true);
                let res = change_completed_task(id, value).await;
                set_change_completed_in_progress.set(false);
                match res {
                    Ok(saved_task) => {
                        checkbox.set_checked(saved_task.completed_at.is_some());

                        if let Some(Ok(tasks)) = tasks_resource.write().as_mut()
                            && let Some(found_task) = tasks.iter_mut().find(|t| t.id == Some(id))
                        {
                            found_task.completed_at = saved_task.completed_at;
                            show_info(
                                "Задача сохранена.".to_owned(),
                                messages,
                            );
                        }
                    }
                    Err(err) => show_server_error(err, messages)
                }
            });
        }
    };

    view! {
        <table class="table is-striped is-fullwidth">
            <thead>
                <tr>
                    <th>{"Приоритет"}</th>
                    <th>{"Завершена"}</th>
                    <th>{"Название"}</th>
                    <th class="is-hidden-mobile">{"Описание"}</th>
                </tr>
            </thead>
            <tbody>
            <Transition fallback=move || view! { <tr>
                                <td colSpan="3" style="text-align: center">
                                    Загрузка...
                                </td>
                            </tr> }>
                {move || tasks_resource.get().map(|data| {
                    let tasks = data.ok().unwrap_or_default();
                    if !tasks.is_empty() {
                        {
                            tasks
                                .iter()
                                .filter(|task| filter_task(task, &filter.get()))
                                .map(|task| {
                                    view! {
                                        <tr>
                                            <td>{task.priority_name()}</td>
                                            <td>
                                                <Checkbox
                                                    class_name="is-medium".to_owned()
                                                    name=format!("completed_{}", task.id.unwrap())
                                                    label="Изменить признак завершения".to_owned()
                                                    value=task.completed_at.is_some()
                                                    title=match &task.completed_at {
                                                        Some(completed_at) => completed_at.to_owned(),
                                                        None => "".to_owned(),
                                                    }
                                                    on:change=completed_on_change
                                                    disabled=move || change_completed_in_progress.get()
                                                />
                                            </td>
                                            <td>
                                                <a
                                                    href=TaskRoutes::details_url(task.id.unwrap())
                                                    aria-label=task.title.to_owned()
                                                >
                                                    {task.title.to_owned()}
                                                </a>
                                            </td>
                                            <td class="is-hidden-mobile">{task.description.to_owned()}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                        }.into_any()
                    } else {
                        view! {
                            <tr>
                                <td colSpan="3" style="text-align: center">
                                    Нет записей
                                </td>
                            </tr>
                        }.into_any()
                    }
                })}
                </Transition>
            </tbody>
        </table>
    }
}
