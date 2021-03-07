pub mod error;
pub mod home;
pub mod users;

use savory::prelude::*;
use savory_elements::prelude::*;
use savory_router::Router;

#[derive(Element, Router)]
#[route(root)]
pub struct Pages {
    #[element(config(required))]
    url: Url,
    #[route(to = "/", persist)]
    home: Option<home::Home>,
    #[route(to = "/user/{id}/", subroute, persist)]
    users: Option<users::Users>,
    error: Option<error::Error>,
    internal: Internal,
}
