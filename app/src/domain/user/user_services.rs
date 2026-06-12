#[cfg(feature = "ssr")]
use crate::common::api_error::ApiError;
#[cfg(feature = "ssr")]
use crate::common::app_state::ssr::use_app_state;
#[cfg(feature = "ssr")]
use crate::domain::home::routing::routes::HomeRoutes;
#[cfg(feature = "ssr")]
use crate::domain::user::user_db::db::*;
#[cfg(feature = "ssr")]
use leptos::context::use_context;
#[cfg(feature = "ssr")]
use leptos::prelude::*;

use leptos::server;
use leptos::server_fn::ServerFnError;

use crate::domain::user::model::create_user_params::CreateUserParams;
use crate::domain::user::model::login_params::LoginParams;
use crate::domain::user::model::user::User;

#[cfg(feature = "ssr")]
pub mod ssr {
    use std::env;

    use actix_web::HttpRequest;
    use chrono::{Duration, Utc};
    use jsonwebtoken::errors::ErrorKind;
    use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
    use leptos::server_fn::ServerFnError;
    use leptos_actix::extract;
    use serde::{Deserialize, Serialize};

    use crate::domain::user::model::user::User;
    use crate::domain::user::routing::routes::UserRoutes;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        exp: usize,
        iat: usize,
        pub user_id: i32,
        pub user_name: String,
    }

    pub fn create_token(user_id: i32, user_name: String) -> Result<String, ServerFnError> {
        let now = Utc::now();
        let exp = now + Duration::hours(7 * 24);

        let claims = Claims {
            user_id,
            user_name,
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?)
    }

    pub fn is_valid_token(token: &str) -> Result<Option<Claims>, ServerFnError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
        match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        ) {
            Ok(data) => Ok(Some(data.claims)),
            Err(e) => {
                let src = e.clone();
                match e.into_kind() {
                    ErrorKind::InvalidToken | ErrorKind::ExpiredSignature => Ok(None),
                    _ => {
                        println!("Error: {}", src);
                        Ok(None)
                    }
                }
            }
        }
    }

    pub async fn get_current_user(redirect_to_login: bool) -> Result<Option<User>, ServerFnError> {
        let req = extract::<HttpRequest>().await?;
        let referer = req
            .headers()
            .get("Referer")
            .and_then(|v| Some(v.to_str().unwrap()))
            .unwrap_or_default();
        if let Some(token_cookie) = req.cookie("todo-token") {
            let claims = is_valid_token(token_cookie.value()).unwrap();
            if let Some(claims) = claims {
                return Ok(Some(User {
                    id: Some(claims.user_id),
                    username: Some(claims.user_name),
                    token: Some(token_cookie.value().to_owned()),
                    ..Default::default()
                }));
            }
        };

        if redirect_to_login {
            leptos_actix::redirect(&UserRoutes::login_url_with_params("", &referer));
        }
        Ok(None)
    }
}

#[server]
pub async fn create_user(params: CreateUserParams) -> Result<User, ServerFnError> {
    use bcrypt::DEFAULT_COST;
    use validator::Validate;

    use crate::domain::user::routing::routes::UserRoutes;

    let validate_result = params.validate();
    if let Err(validation_errors) = validate_result {
        return Err(ApiError::validation(validation_errors))?;
    }

    let app_state = use_app_state().await?;

    if get_user_by_name_from_db(&app_state.pool, params.name.to_owned())
        .await
        .map_err(ServerFnError::new)?
        .is_some()
    {
        return Err(ApiError::validation_field(
            "name",
            "UserAlreadyExist",
            "Пользователь уже существует!",
        ))?;
    }

    let hash_pass = bcrypt::hash(params.password.to_owned().unwrap(), DEFAULT_COST)
        .map_err(|err| ServerFnError::new(format!("Failed hash password: {}", err)))?;

    let user = create_user_in_db(
        &app_state.pool,
        &User { username: params.name.to_owned(), password: Some(hash_pass), ..Default::default() },
    )
    .await
    .map_err(ServerFnError::new)?;

    leptos_actix::redirect(&UserRoutes::login_url_with_params(
        &user.username.to_owned().unwrap(),
        HomeRoutes::base_url(),
    ));

    Ok(user)
}

#[server]
pub async fn login(params: LoginParams) -> Result<User, ServerFnError> {
    use actix_web::http::header::HeaderValue;
    use actix_web::http::header::SET_COOKIE;
    use validator::Validate;

    use self::ssr::*;

    let response_options = use_context::<leptos_actix::ResponseOptions>().unwrap();

    let validate_result = params.validate();
    if let Err(validation_errors) = validate_result {
        return Err(ApiError::validation(validation_errors))?;
    }

    let app_state = use_app_state().await?;

    if let Some(user) = get_user_by_name_from_db(&app_state.pool, params.name.to_owned())
        .await
        .map_err(ServerFnError::new)?
    {
        if bcrypt::verify(params.password.to_owned().unwrap(), &user.password.to_owned().unwrap())
            .map_err(ServerFnError::new)?
        {
            let token = create_token(user.id.unwrap(), user.username.to_owned().unwrap())?;
            update_user_in_db(
                &app_state.pool,
                &User { token: Some(token.to_owned()), ..Default::default() },
            )
            .await
            .map_err(ServerFnError::new)?;

            let cookie_value = format!(
                "todo-token={}; Path=/; HttpOnly; SameSite=Lax; max-age=86400;",
                token.to_owned()
            );
            if let Ok(header) = HeaderValue::from_str(&cookie_value) {
                response_options.insert_header(SET_COOKIE, header);
            }
            leptos_actix::redirect(&HomeRoutes::base_url_with_params(1));

            Ok(User { password: None, ..user })
        } else {
            Err(ServerFnError::new("Неверное имя пользователя или пароль!"))
        }
    } else {
        Err(ServerFnError::new("Не найден пользователь!"))
    }
}

#[server]
pub async fn auth_data() -> Result<User, ServerFnError> {
    let user = ssr::get_current_user(false).await?;
    match user {
        Some(user) => Ok(user),
        None => Ok(User::default()),
    }
}

#[server]
pub async fn logout() -> Result<bool, ServerFnError> {
    use actix_web::http::header::HeaderValue;
    use actix_web::http::header::SET_COOKIE;

    use self::ssr::*;

    if get_current_user(false).await?.is_some() {
        let response_options = use_context::<leptos_actix::ResponseOptions>().unwrap();

        let app_state = use_app_state().await?;

        if let Some(user) = ssr::get_current_user(false).await? {
            update_user_in_db(
                &app_state.pool,
                &User { id: user.id, token: None, ..Default::default() },
            )
            .await
            .map_err(ServerFnError::new)?;
        }

        let cookie_value = "todo-token=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
        if let Ok(header) = HeaderValue::from_str(cookie_value) {
            response_options.insert_header(SET_COOKIE, header);
        }
        leptos_actix::redirect(&HomeRoutes::base_url_with_params(2));

        return Ok(true);
    }
    Ok(false)
}
