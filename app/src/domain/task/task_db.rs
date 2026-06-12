#[cfg(feature = "ssr")]
pub mod db {

    use chrono::{DateTime, FixedOffset};
    use sqlx::query_as;

    use crate::common::DbPool;

    #[derive(Debug, sqlx::FromRow)]
    pub struct TaskInDb {
        pub id: Option<i32>,
        pub title: Option<String>,
        pub description: Option<String>,
        pub priority: Option<String>,
        pub completed_at: Option<DateTime<FixedOffset>>,
    }

    pub async fn get_tasks_from_db(
        pool: &DbPool,
        user_id: Option<i32>,
    ) -> Result<Vec<TaskInDb>, sqlx::Error> {
        query_as!(
            TaskInDb,
            r#"
                SELECT
                    id AS "id!: i32",
                    title,
                    description,
                    priority,
                    completed_at AS "completed_at!: Option<DateTime<FixedOffset>>"
                FROM tasks
                WHERE deleted_at is null and user_id=$1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_task_from_db(
        pool: &DbPool,
        id: i32,
        user_id: Option<i32>,
    ) -> Result<Option<TaskInDb>, sqlx::Error> {
        query_as!(
            TaskInDb,
            r#"
                SELECT
                    id AS "id!: i32",
                    title,
                    description,
                    priority,
                    completed_at AS "completed_at!: Option<DateTime<FixedOffset>>"
                FROM tasks
                    WHERE id = $1 and deleted_at is null and user_id=$2
                "#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn get_task_by_title_from_db(
        pool: &DbPool,
        title: &Option<String>,
        user_id: i32,
    ) -> Result<Option<TaskInDb>, sqlx::Error> {
        let title = title.to_owned().unwrap();
        query_as!(
            TaskInDb,
            r#"
                SELECT
                    id AS "id!: i32",
                    title,
                    description,
                    priority,
                    completed_at AS "completed_at!: Option<DateTime<FixedOffset>>"
                FROM tasks
                    WHERE title = $1 and deleted_at is null and user_id=$2
                "#,
            title,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete_task_in_db(
        pool: &DbPool,
        id: i32,
        user_id: Option<i32>,
    ) -> Result<i32, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                    DELETE FROM tasks
                    WHERE id = $1 and user_id=$2
                    RETURNING id
                "#,
            id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.id.try_into().unwrap())
    }

    pub async fn update_task_in_db(
        pool: &DbPool,
        patch: &TaskInDb,
        user_id: Option<i32>,
    ) -> Result<TaskInDb, sqlx::Error> {
        let result = sqlx::query_as!(
            TaskInDb,
            r#"
                    UPDATE tasks
                    SET title=$1,
                        description=$2,
                        priority=$3,
                        completed_at=$4
                    WHERE id = $5 and user_id=$6
                    RETURNING id AS "id!: i32", title, description, priority, completed_at AS "completed_at!: Option<DateTime<FixedOffset>>"
            "#,
            patch.title,
            patch.description,
            patch.priority,
            patch.completed_at,
            patch.id,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn create_task_in_db(
        pool: &DbPool,
        task: &TaskInDb,
        user_id: i32,
    ) -> Result<TaskInDb, sqlx::Error> {
        query_as!(
            TaskInDb,
            r#"
                INSERT INTO tasks (title, description, priority, completed_at, user_id)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id AS "id!: i32", title, description, priority, completed_at AS "completed_at!: Option<DateTime<FixedOffset>>"
            "#,
            task.title,
            task.description,
            task.priority,
            task.completed_at,
            user_id
        )
        .fetch_one(pool)
        .await
    }
}
