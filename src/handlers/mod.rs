mod post_data;
mod get_data;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_data::post_data);
    cfg.service(get_data::get_data);
}
