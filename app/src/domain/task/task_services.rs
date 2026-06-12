#[cfg(feature = "ssr")]
use chrono::Datelike;
use leptos::server;
use leptos::server_fn::ServerFnError;

use crate::components::ui::select_input::SelectOption;
use crate::domain::task::model::task::Task;

#[cfg(feature = "ssr")]
use crate::domain::task::task_db::db::TaskInDb;
#[cfg(feature = "ssr")]
use leptos::prelude::*;

#[server]
pub async fn get_task(id: i32) -> Result<Task, ServerFnError> {
    use super::task_db::db::get_task_from_db;
    use crate::common::api_error::ApiError;
    use crate::common::app_state::ssr::*;
    use crate::domain::user::user_services::ssr::get_current_user;

    //tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    if let Some(user) = get_current_user(true).await? {
        let app_state = use_app_state().await?;

        if let Some(task) =
            get_task_from_db(&app_state.pool, id, user.id).await.map_err(ServerFnError::new)?
        {
            return Ok(task.into());
        } else {
            return Err(ApiError::NotFound("Задача не найдена!".to_owned()))?;
        }
    }

    Ok(Task::default())
}

#[server]
pub async fn delete_task(id: i32) -> Result<bool, ServerFnError> {
    use super::task_db::db::*;
    use crate::common::app_state::ssr::*;
    use crate::domain::user::user_services::ssr::get_current_user;

    if let Some(user) = get_current_user(true).await? {
        let app_state = use_app_state().await?;

        delete_task_in_db(&app_state.pool, id, user.id).await.map_err(ServerFnError::new)?;

        leptos_actix::redirect("/");
        return Ok(true);
    }

    Ok(false)
}

#[server]
pub async fn get_tasks(
    filter: Option<String>,
    sort_kind: Option<String>,
) -> Result<Vec<Task>, ServerFnError> {
    use super::task_db::db::get_tasks_from_db;
    use crate::common::app_state::ssr::*;
    use crate::domain::task::model::task::{filter_task, sort_task};
    use crate::domain::user::user_services::ssr::get_current_user;

    //tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    if let Some(user) = get_current_user(false).await? {
        let app_state = use_app_state().await?;

        let mut tasks: Vec<Task> = get_tasks_from_db(&app_state.pool, user.id)
            .await
            .map_err(ServerFnError::new)?
            .into_iter()
            .map(|t| t.into())
            .collect();

        if filter.is_some() {
            tasks = tasks.into_iter().filter(|t| filter_task(t, &filter)).collect::<Vec<Task>>();
        }

        if sort_kind.is_some() {
            tasks.sort_by(|task1, task2| sort_task(task1, task2, &sort_kind));
        }

        return Ok(tasks);
    }
    Ok(vec![])
}

#[server]
pub async fn update_or_create_task(task: Task) -> Result<Task, ServerFnError> {
    use validator::Validate;

    use super::task_db::db::create_task_in_db;
    use super::task_db::db::get_task_by_title_from_db;
    use super::task_db::db::update_task_in_db;
    use crate::common::api_error::ApiError;
    use crate::common::app_state::ssr::*;
    use crate::domain::user::user_services::ssr::get_current_user;

    if let Some(user) = get_current_user(true).await? {
        let app_state = use_app_state().await?;

        let validate_result = task.validate();
        if let Err(validation_errors) = validate_result {
            return Err(ApiError::validation(validation_errors))?;
        }

        if let Some(found_task) =
            get_task_by_title_from_db(&app_state.pool, &task.title, user.id.unwrap())
                .await
                .map_err(ServerFnError::new)?
            && found_task.id != task.id
        {
            return Err(ApiError::validation_field(
                "title",
                "TaskAlreadyExist",
                "Задача с таким названием уже существует!",
            ))?;
        }

        let patch = Task { ..task }.fix_completed_at().to_owned();

        let saved_task = if task.id.is_some() {
            update_task_in_db(
                &app_state.pool,
                &(patch).into(),
                user.id,
            )
            .await
            .map_err(ServerFnError::new)?
        } else {
            create_task_in_db(
                &app_state.pool,
                &(patch).into(),
                user.id.unwrap(),
            )
            .await
            .map_err(ServerFnError::new)?
        };

        leptos_actix::redirect(&format!("/task/{}", saved_task.id.unwrap()));
        return Ok((saved_task).into());
    }

    Ok(task)
}

#[server]
pub async fn change_completed_task(id: i32, completed: bool) -> Result<Task, ServerFnError> {
    use super::task_db::db::get_task_from_db;
    use super::task_db::db::update_task_in_db;
    use crate::common::api_error::ApiError;
    use crate::common::app_state::ssr::*;
    use crate::domain::user::user_services::ssr::get_current_user;

    if let Some(user) = get_current_user(true).await? {
        let app_state = use_app_state().await?;

        if let Some(mut task) =
            get_task_from_db(&app_state.pool, id, user.id).await.map_err(ServerFnError::new)?
        {
            task.completed_at = match completed {
                true => Some(chrono::Utc::now().fixed_offset()),
                false => None,
            };

            let saved_task = update_task_in_db(&app_state.pool, &task, user.id)
                .await
                .map_err(ServerFnError::new)?;

            return Ok((saved_task).into());
        } else {
            return Err(ApiError::NotFound("Задача не найдена!".to_owned()))?;
        }
    }

    Ok(Task::default())
}

#[server]
pub async fn get_priorities() -> Result<Vec<SelectOption>, ServerFnError> {
    Ok(vec![
        (Some("C".to_owned()), Task::priority_by_name("C")),
        (Some("H".to_owned()), Task::priority_by_name("H")),
        (Some("N".to_owned()), Task::priority_by_name("N")),
        (Some("L".to_owned()), Task::priority_by_name("L")),
    ])
}

pub async fn get_filter_options() -> Result<Vec<SelectOption>, ServerFnError> {
    Ok(vec![filter_to_option("Completed".to_owned()), filter_to_option("Uncompleted".to_owned())])
}

fn filter_to_option(filter: String) -> SelectOption {
    match filter.as_str() {
        "Completed" => (Some(filter), "Завершенные".to_owned()),
        "Uncompleted" => (Some(filter), "Незавершенные".to_owned()),
        _ => (None, "Не выбран".to_owned()),
    }
}

pub async fn get_sort_options() -> Result<Vec<SelectOption>, ServerFnError> {
    Ok(vec![sort_to_option("Title".to_owned()), sort_to_option("Priority".to_owned())])
}

fn sort_to_option(sort_kind: String) -> SelectOption {
    match sort_kind.as_str() {
        "Title" => (Some(sort_kind), "Название".to_owned()),
        "Priority" => (Some(sort_kind), "Приоритет".to_owned()),
        _ => (None, "Не выбран".to_owned()),
    }
}

#[cfg(feature = "ssr")]
impl From<TaskInDb> for Task {
    fn from(task: TaskInDb) -> Self {
        let completed_at = match task.completed_at {
            Some(completed_at) => { 
                if completed_at.year() == 1950 {
                    None
                } else {
                    Some(completed_at.to_rfc2822()) 
                }
            },
            None => None,
        };
        Task {
            id: task.id,
            title: task.title.to_owned(),
            description: task.description.to_owned(),
            priority: task.priority.to_owned(),
            completed_at: completed_at,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<Task> for TaskInDb {
    fn from(task: Task) -> Self {
        let completed_at = match &task.completed_at {
            Some(completed_at) => Some(
                <chrono::DateTime<chrono::FixedOffset>>::parse_from_rfc2822(&completed_at).unwrap(),
            ),
            None => None,
        };

        TaskInDb {
            id: task.id,
            title: task.title.to_owned(),
            description: task.description.to_owned(),
            priority: task.priority.to_owned(),
            completed_at: completed_at,
        }
    }
}
