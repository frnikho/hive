use std::io::Result;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
use actix_web::middleware::Logger;
use crate::handlers::handler::Handler;
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

pub mod state;

const DEFAULT_HOST: &str = "0.0.0.0";
const DEFAULT_PORT: u16 = 8080;
const DEFAULT_WORKERS: usize = 4;


#[actix_web::main]
async fn main() -> Result<()> {
    let state = AppState::new()?;
    let _config = state.config.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(state.clone()))
            .configure(handlers::user_handler::UserHandler::route)
            .configure(handlers::devices_handler::DeviceHandler::route)
    })
        .workers(DEFAULT_WORKERS)
        .bind((DEFAULT_HOST.to_string(), DEFAULT_PORT))?
        .run()
        .await
}
