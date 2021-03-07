pub mod error;
pub mod user;

use savory::prelude::*;
use savory_elements::prelude::*;
use savory_router::Router;

#[derive(Element, Router)]
pub struct Users {
    #[element(config(required))]
    url: Url,
    #[element(config(required))]
    #[route(param)]
    id: usize,
    #[route(to = "/", persist)]
    user: Option<user::User>,
    error: Option<error::Error>,
    internal: Internal,
}
