use rocket::{delete, get, post, put, Route, routes, State};
use rocket::http::Status;
use rocket::serde::json::Json;

pub(crate) fn routes() -> Vec<Route> {
    routes![route]
}

#[post("/route")]
async fn route() -> Status {
  Status::Ok
}