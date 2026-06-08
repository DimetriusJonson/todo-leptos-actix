use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

use crate::components::layout::message_banner::{Messages, show_error, show_info};
use crate::components::ui::button::Button;
use crate::components::ui::button_link::ButtonLink;
use crate::domain::home::routing::routes::HomeRoutes;
use crate::domain::user::routing::routes::UserRoutes;
use crate::domain::user::user_services::{Logout, auth_data};

#[component]
pub fn Navbar() -> impl IntoView {
    let (nav_links_active, set_nav_links_active) = signal(false);

    let messages = use_context::<Messages>().expect("Cant get messages context!");

    let logout = ServerAction::<Logout>::new();

    let query_map = use_query_map();
    let auth = move || query_map.with(|m| m.get("auth"));

    let user_resource = Resource::new_blocking(auth, |_s| async move { auth_data().await });
    provide_context(user_resource);

    Effect::new(move |_| {
        if let Some(res) = logout.value().get() {
            match res {
                Ok(_) => {
                    show_info("Вы вышли!".to_owned(), messages);
                    logout.clear();
                }
                Err(err) => show_error(err.to_string(), messages),
            }
        }
    });

    view! {
        <nav class="navbar is-primary" aria-label="main navigation">
            <div class="navbar-brand">
                <a
                    class="navbar-item is-size-3 has-text-weight-extrabold is-family-code mx-1"
                    href=HomeRoutes::base_url()>{HomeRoutes::label()}</a>

                <a
                    role="button"
                    class="navbar-burger"
                    aria-label="menu"
                    aria-expanded="false"
                    on:click=move |_| set_nav_links_active.set(!nav_links_active.get())
                    href=HomeRoutes::base_url()>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            </div>

            <div
                class:is-active=move || nav_links_active.get()
                class={"navbar-menu no-script-navbar-menu"}
                id="nav-links"
            >
                <div class="navbar-start">
                    <div class="navbar-item">
                        <ButtonLink label="Пользователи".to_owned() href="/users".to_owned()/>
                    </div>
                </div>

                <div class="navbar-end">
                    <div class="buttons">
                        <Transition>
                            {move || user_resource.get().map(|data| {
                                let user = data.ok().unwrap_or_default();
                                if let Some(user_name)=user.username {
                                        view! {
                                            <div class="navbar-item">
                                                <ActionForm action=logout>
                                                    <Button
                                                        class_name="is-warning is-light".to_owned()
                                                        label={format!("Выйти {}", user_name)}
                                                        loading=move || logout.pending().get()
                                                        disabled=move || logout.pending().get()
                                                        on_click=move |_| {}
                                                    />
                                                </ActionForm>
                                            </div>
                                        }.into_any()
                                } else {
                                        view! {
                                            <div class="navbar-item">
                                                    <ButtonLink
                                                        class_name="button is-warning is-soft is-rounded".to_owned()
                                                        label="Создать пользователя".to_owned()
                                                        href=UserRoutes::create_url().to_owned()
                                                    />
                                                </div>
                                                <div class="navbar-item">
                                                    <ButtonLink
                                                        class_name="is-light".to_owned()
                                                        label="Войти".to_owned()
                                                        href=UserRoutes::login_url().to_owned()
                                                    />
                                                </div>
                                        }.into_any()
                                }
                            })}
                        </Transition>
                    </div>
                </div>
            </div>
        </nav>

        <noscript>
            <style>
                r#"
                .no-script-navbar-menu {
                    display: block;
                }
                "#
            </style>
        </noscript>


    }
}
