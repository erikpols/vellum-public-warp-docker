extern crate dotenv;

use env_logger::*;
use log::LevelFilter;
use std::convert::Infallible;
use std::io::Write;
use warp::{http::StatusCode, Filter, Rejection, Reply};

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

    let index = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file("./static/index.html"));
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

    let log = warp::log::custom(|info| {
        eprintln!(
            "{} {} {} {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
        );
    });

    let routes = index.with(cors).with(log).recover(handle_rejection);

    log::warn!("Launching warp on port 3022");
    warp::serve(routes).run(([0, 0, 0, 0], 3022)).await;
}
