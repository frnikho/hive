use actix_web::web;

pub trait Handler {
    fn route(cfg: &mut web::ServiceConfig);
}
