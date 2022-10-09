mod api;
mod model;
pub mod repo;

use api::task::{complete_task, fail_task, get_task, pause_task, start_task, submit_task};

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use aws_config::SdkConfig;
use repo::ddb::DDBRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config: SdkConfig = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let ddb_repo: DDBRepo = DDBRepo::init(
            String::from("task"), 
            config.clone()
        );
        let ddb_data: Data<DDBRepo> = Data::new(ddb_repo);

        let logger: Logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
            .service(submit_task)
            .service(start_task)
            .service(complete_task)
            .service(pause_task)
            .service(fail_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
