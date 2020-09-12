use chrono::prelude::*;
use clap::{App, Arg};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use warp::{http::Response, Filter};

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const GIT_COMMIT_HASH: &str = if let Some(hash) = built_info::GIT_COMMIT_HASH {
    hash
} else {
    ":-("
};

fn is_num(s: String) -> Result<(), String> {
    if let Err(..) = s.parse::<u16>() {
        return Err(String::from("Not a valid port number!"));
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = App::new("demo")
        .version(format!("{} {}", env!("CARGO_PKG_VERSION"), GIT_COMMIT_HASH).as_ref())
        .arg(
            Arg::with_name("port")
                .default_value("80")
                .help("listening port")
                .long("port")
                .validator(is_num)
                .required(true),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

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
    let short_hash = if GIT_COMMIT_HASH.len() > 7 {
        &GIT_COMMIT_HASH[0..7]
    } else {
        ""
    };
    Ok(Response::builder()
        .header(
            "X-App",
            format!(
                "{}:{}:{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                short_hash
            ),
        )
        .body(GIT_COMMIT_HASH))
}
