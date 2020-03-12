//! Traits for making elements routable

pub trait Routable {
    type Route;
    fn from_url(url: &seed::Url) -> Option<Self::Route>;
    fn route(&self) -> Self::Route;
    // TODO: make this trait simple by adding function that define the url
    // schema used by this Routable object and give from_url(url) for free!!
}
