use std::io::Result;
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::{App, HttpServer};
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::middleware::Logger;
use env_logger::Env;
use crate::handlers::handler::Handler;
use crate::providers::cache::create_super_user_token;
use crate::state::AppState;

pub mod models;
pub mod handlers;
pub mod extractors;
pub mod exceptions;
pub mod repositories;
pub mod services;
pub mod templates;
pub mod dtos;
pub mod providers;
pub mod entities;
pub mod utils;

pub mod state;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_WORKERS: usize = 4;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let state = AppState::new()?;
    let config = state.config.clone();
    let store = RedisSessionStore::new(config.api_session_url.clone()).await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Error loading redis session store"))?;

    create_super_user_token(&mut state.cache.get_connection().unwrap())?;

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                store.clone(),
                Key::from(config.api_session_secret.clone().as_bytes())
            ))
            .wrap(Logger::default())
            .app_data(Data::new(state.clone()))
            .configure(handlers::user_handler::UserHandler::route)
            .configure(handlers::auth_handler::AuthHandler::route)
            .configure(handlers::devices_handler::DeviceHandler::route)
            .configure(handlers::system_handler::SystemHandler::route)

    })
        .workers(DEFAULT_WORKERS)
        .bind((DEFAULT_HOST.to_string(), DEFAULT_PORT))?
        .run()
        .await
}
