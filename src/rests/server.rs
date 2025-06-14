use std::net::Ipv4Addr;
use warp::Filter;
use warp::http::Method;
use crate::models::rest::DataTransport;
use crate::rests::handlers::{get_post, login};



fn json_body() -> impl Filter<Extract = (DataTransport,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1_024 * 10).and(warp::body::json())
}

pub async fn start(ip : Ipv4Addr, port :  u16) {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
        ])
        .allow_methods(&[Method::PUT, Method::DELETE, Method::POST, Method::GET])
        .build();


    let login = warp::post()
        .and(warp::path("v5"))
        .and(warp::path("pocket"))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(login);


    let routes = login.with(cors);
    
    

    println!("Server started at http://{}:{}", ip, port);
    warp::serve(routes).run((ip, port)).await;
}