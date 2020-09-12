use chrono::prelude::*;
use lazy_static::lazy_static;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use warp::{http::Response, Filter};

lazy_static! {
    static ref COMMIT: String = match env::var("COMMIT") {
        Ok(val) => val,
        Err(_) => "not defiend".to_string(),
    };
}

#[tokio::main]
async fn main() {
    // port where app will listen
    let port = 80;

    let now = Utc::now();
    println!(
        "{} - Listening on *:{}",
        now.to_rfc3339_opts(SecondsFormat::Secs, true),
        port
    );

    // define the routes to use
    let hello = warp::get().and_then(hello);
    let health = warp::any().and(warp::path("health")).and_then(health);

    // GET /*
    // ANY /health
    let routes = health.or(hello);

    // listen in both tcp46 falling back to IPv4
    let addr = match IpAddr::from_str("::0") {
        Ok(a) => a,
        Err(_) => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    };

    // start service
    warp::serve(routes).run((addr, port)).await;
}

// GET  /*
async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(Response::builder().body("hello world!"))
}

// ANY /health
// return X-APP header and the commit in the body
async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(Response::builder()
        .header(
            "X-App",
            format!("{}:{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
        .body(format!("commit: {}", &*COMMIT)))
}
