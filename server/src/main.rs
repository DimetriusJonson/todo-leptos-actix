#![recursion_limit = "256"]

use std::{env, thread};

use actix_files::Files;
use actix_web::{App, HttpServer, middleware, web};
use app::app::shell;
use dotenv::dotenv;
use leptos::prelude::*;
use leptos_actix::{LeptosRoutes, generate_route_list};
use log::{error, info};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

mod db;
//mod fallback;

use crate::db::create_pool;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let env_file_name = format!(".env.{}", environment);
    println!("environment={}, env_file_name={}", environment, env_file_name);

    dotenv().ok();
    dotenvy::from_filename_override(env_file_name).ok();

    LogTracer::init().expect("Failed to set logger");

    let subscriber = FmtSubscriber::builder()
        .with_ansi(true)
        //.with_file(true)
        .with_line_number(true)
        // Apply the EnvFilter to use RUST_LOG
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set subscriber");

    match thread::available_parallelism() {
        Ok(n) => info!("Available parallelism: {}", n),
        Err(e) => error!("Error getting parallelism: {}", e),
    }

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;

    let pool = create_pool().await?;

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(app::app::App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        println!("listening on http://{}", &addr);

        let app = App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the favicon from /favicon.ico
            //.service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            })
            .app_data(web::Data::new(pool.clone()));

        app.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
