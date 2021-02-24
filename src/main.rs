use chrono::prelude::*;
use clap::{App, Arg};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use warp::{http::HeaderMap, http::Response, Filter};

const HTML: &str = r#"
<html><head><style>
body {
    background-color: #87CEFA;
}
</style></head><body>
Yukgaejang :)!</br>
</body></html>"#;

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
    let hello = warp::get().and(log_headers()).and_then(hello);
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

fn log_headers() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    warp::header::headers_cloned()
        .map(|headers: HeaderMap| {
            let mut header_hashmap: HashMap<String, String> = HashMap::new();
            for (k, v) in headers.iter() {
                let k = k.as_str().to_owned();
                let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
                header_hashmap.entry(k).or_insert(v);
            }
            let j = serde_json::to_string(&header_hashmap).unwrap();
            println!("{}", j);
        })
        .untuple_one()
}

// GET  /*
async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(Response::builder().body(HTML))
    //let blue = HTML.replace("#87CEFA", "green");
    //Ok(Response::builder().body(blue))
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
