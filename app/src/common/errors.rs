use leptos::prelude::*;
use std::error::Error;
use std::fmt::Display;

#[cfg(feature = "ssr")]
use leptos_actix::ResponseOptions;

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    InternalServerError,
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

impl AppError {
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound(_) => 404,
            AppError::InternalServerError => 500,
        }
    }
}

/// Renders errors and sets HTTP status code on SSR. Used in `server/src/fallback.rs`.
#[component]
pub fn ErrorBoundary(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors_signal: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors_signal = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors_signal {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    let errors = errors_signal.get();

    let errors: Vec<AppError> =
        errors.into_iter().filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned()).collect();
    println!("Errors: {errors:#?}");

    #[cfg(feature = "ssr")]
    let response = use_context::<ResponseOptions>();
    #[cfg(feature = "ssr")]
    if let Some(response) = response
        && !errors.is_empty()
    {
        use actix_web::http::StatusCode;

        response.set_status(StatusCode::from_u16(errors[0].status_code()).unwrap());
    }

    view! {
        <h1>{if errors.len() > 1 { "Errors" } else { "Error" }}</h1>
        <For
            each=move || { errors.clone().into_iter().enumerate() }
            key=|(index, _error)| *index
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code();
                view! {
                    <h2>{error_code.to_string()}</h2>
                    <p>"Error: " {error_string}</p>
                }
            }
        />
    }
}
