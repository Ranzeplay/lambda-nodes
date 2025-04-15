pub mod logs;
pub mod nodes;
pub mod pipelines;
pub mod ping;
pub mod exec;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(logs::configure)
            .configure(nodes::configure)
            .configure(pipelines::configure)
    );

    cfg.service(ping::ping);

    cfg.service(exec::exec);
}