extern crate dotenv;

use env_logger::*;
use log::LevelFilter;
use std::io::Write;
use warp::{http::StatusCode, Rejection, Reply, Filter};
// use futures_util::{StreamExt};
use std::convert::Infallible;

// mod error;
// mod caches;
// mod constants;
// mod controllers;
// mod handlers;
// mod integrations;
// mod models;
// mod db;
// mod utilities;
use std::env;


pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    // let json = warp::reply::json(&ErrorResponse {
    //     message: message.into(),
    // });

    Ok(warp::reply::with_status(message, code))
}


#[tokio::main]
async fn main() {
    // init logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}]: {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Warn)
        .init();

        let index = warp::path::end().and(warp::get()).and(warp::fs::file("./static/index.html"));
        // let index = path!("/").and(warp::fs::file("./static/index.html"));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Content-Type",
        ])
        .allow_methods(vec!["GET", "POST"]);

    // let log = warp::log("example::api");
    let log = warp::log::custom(|info| {
        eprintln!(
            "{} {} {} {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
        );
    });

    let routes = index
        .with(cors)
        .with(log)
        .recover(handle_rejection);

    //     let port = if let Ok(port_str) = env::var("PORT") {
    //         if let Ok(port) = port_str.parse::<u16>() {
    //             log::warn!("Starting server on port {}", port);
    //             port
    //         } else {
    //             log::warn!("Could not parse environment variable PORT. Assuming dev environment: port 3022");
    //             3022    
    //         }
    //     } else {
    //         log::warn!("Environment variable PORT not found. Assuming dev environment: port 3022");
    //         3022
    //     };
        // let port = env::var("PORT").unwrap_or("".to_string()).parse::<u16>().unwrap_or(3022);
        log::warn!("Launching warp on port 3022");

    warp::serve(routes).run(([0, 0, 0, 0], 3022)).await;
}
