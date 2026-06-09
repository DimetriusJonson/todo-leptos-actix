use std::time::Duration;

use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, RoutingProgress};
use leptos_router::{StaticSegment, path};

use crate::components::layout::message_banner::MessageBanner;
use crate::components::layout::navbar::Navbar;
use crate::domain::home::routing::home_page::HomePage;
use crate::domain::task::routing::routes::TaskRoutes;
use crate::domain::task::routing::task_edit_page::TaskEditPage;
use crate::domain::task::routing::task_page::TaskPage;
use crate::domain::user::routing::create_user_page::CreateUserPage;
use crate::domain::user::routing::login_page::LoginPage;
use crate::domain::user::routing::routes::UserRoutes;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="text-scale" content="scale" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (is_routing, set_is_routing) = signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="bulma" href="/bulma.min.css" />
        <Stylesheet id="leptos" href="/pkg/todo_leptos.css" />

        <Title text="TODO leptos app"/>
        <Meta name="keywords" content="todo, leptos, rust, web application, development" />
        <Meta name="description" content="Пример приложения списка дел (TODO) с использованием языка программирования Rust и фреймворка Leptos." />

        <Router set_is_routing>

            <div class="progress-container pt-0 mt-0">
                <RoutingProgress is_routing max_time=Duration::from_millis(250) />
            </div>

            <section class="section p-0">
                <div class="is-paddingless">
                    <main>
                        <MessageBanner />
                        <Navbar />

                        <ErrorBoundary fallback=move |errors| view! {
                            <section class="section">
                                <div class="box has-text-centered">
                                    <div class="title is-size-1 has-text-danger">500</div>
                                    <ul>
                                        {move || errors.get()
                                            .into_iter()
                                            .map(|(_, error)| view! { <li>{error.to_string()}</li> })
                                            .collect::<Vec<_>>()
                                        }
                                    </ul>
                                </div>
                            </section> }>
                            <Routes transition=true fallback=NotFound>
                                <ParentRoute path=path!("/") view=Outlet>

                                    <ParentRoute path=StaticSegment(UserRoutes::base_segment()) view=Outlet>
                                        <Route path=StaticSegment(UserRoutes::create_segment()) view=CreateUserPage />
                                        <Route path=StaticSegment(UserRoutes::login_segment()) view=LoginPage />
                                    </ParentRoute>

                                    <ParentRoute path=StaticSegment(TaskRoutes::base_segment()) view=Outlet>
                                        <Route path=StaticSegment(TaskRoutes::create_segment()) view=TaskEditPage />
                                        <Route path=path!(":id") view=TaskPage />
                                        <Route path=path!(":id/edit") view=TaskEditPage />
                                    </ParentRoute>

                                    <Route path=path!("") view=HomePage />

                                </ParentRoute>

                                <Route path=path!("/*any") view=NotFound />

                            </Routes>
                        </ErrorBoundary>

                    </main>
                </div>
            </section>
        </Router>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <section class="section">
            <div class="box has-text-centered">
                <div class="title is-size-1 has-text-danger">404</div>
                <div class="subtitle">Страница не найдена</div>
            </div>
        </section>
    }
}
