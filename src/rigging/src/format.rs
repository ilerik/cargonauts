use futures::{Future, Stream};

use {ResourceEndpoint, Error};
use environment::Environment;
use method::Method;
use request::Request;
use http;

pub trait Format<T: ResourceEndpoint, M: ?Sized + Method<T>> {
    type ReqFuture: Future<Item = <M::Request as Request<T>>::BodyParts, Error = Error> + 'static;

    fn receive_request(req: http::Request, env: &Environment) -> Self::ReqFuture;

    fn present_unit<F>(future: F, template: Option<Template>, env: &Environment) -> http::BoxFuture
        where F: Future<Item = (), Error = Error> + 'static;

    fn present_resource<F>(future: F, template: Option<Template>, env: &Environment) -> http::BoxFuture
        where F: Future<Item = M::Response, Error = Error> + 'static;

    fn present_collection<S>(stream: S, template: Option<Template>, env: &Environment) -> http::BoxFuture
        where S: Stream<Item = M::Response, Error = Error> + 'static;

    fn present_error(error: Error, env: &Environment) -> http::BoxFuture;
}

#[derive(Copy, Clone)]
pub struct Template {
    src: &'static str
}

impl Template {
    #[doc(hidden)]
    pub fn static_prepare(src: &'static str) -> Template {
        Template { src }
    }
}

impl AsRef<str> for Template {
    fn as_ref(&self) -> &str { self.src }
}
