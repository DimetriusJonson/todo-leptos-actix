#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::common::DbPool;
    use actix_web::web;
    use leptos::prelude::{LeptosOptions, *};

    #[derive(Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub pool: DbPool,
    }

    pub async fn use_app_state() -> Result<web::Data<AppState>, ServerFnError> {
        leptos_actix::extract::<web::Data<AppState>>().await.map_err(ServerFnError::new)
    }
}
