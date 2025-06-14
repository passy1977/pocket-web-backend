use warp::Filter;
use warp::reject::Reject;
use crate::models::rest::DataTransport;

#[derive(Debug)]
pub struct FailureReason {
    pub reason: &'static str,
}
impl Reject for FailureReason {}
impl std::fmt::Display for FailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {}", self.reason)
    }
}


// A function to handle GET requests at /posts/{id}
pub async fn get_post(request: DataTransport) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post


    let ret = DataTransport::new();


    Ok(warp::reply::json(&ret))
}

pub(super) async fn login(request: DataTransport) -> Result<impl warp::Reply, warp::Rejection> {

    match crate::controllers::rests_controller::login(request) {
        Ok(response) => Ok(warp::reply::json(&response)),
        Err(err) =>  Err(warp::reject::custom(FailureReason { reason: err }))
    }
    
}