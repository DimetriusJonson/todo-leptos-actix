#[cfg(feature = "ssr")]
pub mod db {

    use sqlx::query_as;

    use crate::common::DbPool;
    use crate::domain::task::model::task::Task;

    pub async fn get_tasks_from_db(
        pool: &DbPool,
        user_id: Option<i64>,
    ) -> Result<Vec<Task>, sqlx::Error> {
        query_as!(
            Task,
            r#"
                SELECT
                    id,
                    title,
                    description,
                    priority,
                    completed_at
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
        id: i64,
        user_id: Option<i64>,
    ) -> Result<Option<Task>, sqlx::Error> {
        query_as!(
            Task,
            r#"
                SELECT
                    id,
                    title,
                    description,
                    priority,
                    completed_at
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
        user_id: i64,
    ) -> Result<Option<Task>, sqlx::Error> {
        query_as!(
            Task,
            r#"
                SELECT
                    id,
                    title,
                    description,
                    priority,
                    completed_at
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
        id: i64,
        user_id: Option<i64>,
    ) -> Result<i64, sqlx::Error> {
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

        Ok(result.id)
    }

    pub async fn update_task_in_db(
        pool: &DbPool,
        patch: &Task,
        user_id: Option<i64>,
    ) -> Result<Task, sqlx::Error> {
        let result = sqlx::query_as!(
            Task,
            r#"
                    UPDATE tasks
                    SET title=$1,
                        description=$2,
                        priority=$3,
                        completed_at=$4
                    WHERE id = $5 and user_id=$6
                    RETURNING id, title, description, priority, completed_at
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
        task: &Task,
        user_id: i64,
    ) -> Result<Task, sqlx::Error> {
        query_as!(
            Task,
            r#"
                INSERT INTO tasks (title, description, priority, completed_at, user_id)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, title, description, priority, completed_at
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
