#[macro_use]
extern crate log;

use actix_web::{get, middleware, App, HttpServer, Responder};
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::env;
use std::io::Write;

#[get("/")]
async fn index() -> impl Responder {
    format!("Welcome to the Republic!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env::set_var("RUST_BACKTRACE", "1");
    init_logger();
    HttpServer::new(|| App::new().wrap(middleware::Logger::default()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}:{}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            formatter.default_styled_level(record.level()),
            record.target(),
            record.line().unwrap_or(0),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
    }

    builder.init();

    info!("env_logger initialized.");
}
