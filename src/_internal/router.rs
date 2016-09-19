use api;
use Serialize;
use router::Router as RouterTrait;
use router::{Status, Response};
use _internal::{self, ResourceDocument, CollectionDocument};

pub struct Router<'a, R: RouterTrait + 'a> {
    router: &'a mut R,
}

impl<'a, R: RouterTrait> Router<'a, R> {
    pub fn new(router: &'a mut R) -> Router<'a, R> {
        Router {
            router: router,
        }
    }

    pub fn attach_get<T: api::Get>(&mut self) where _internal::Resource<T>: _internal::Wrapper<T> {
        self.router.attach_get(T::resource(), |id, includes| {
            let mut response = R::Response::default();
            let id = match id.parse() {
                Ok(id)  => id,
                Err(_)  => {
                    response.set_status(Status::BadRequest);
                    return response
                }
            };
            if let Some(resource) = T::get(id) {
                let document = ResourceDocument::new(resource, includes);
                match document.serialize(response.serializer()) {
                    Ok(_)   => response.set_status(Status::Ok),
                    // TODO write the error to the body in the error case
                    Err(_)  => response.set_status(Status::InternalError),
                }
                response
            } else {
                // TODO write the error to the body in the error case
                response.set_status(Status::NotFound);
                response
            }
        });
    }

    pub fn attach_index<T: api::Index>(&mut self) where _internal::Resource<T>: _internal::Wrapper<T> {
        self.router.attach_index(T::resource(), |includes| {
            let mut response = R::Response::default();
            let resources = T::index();
            let document = CollectionDocument::new(resources, includes);
            match document.serialize(response.serializer()) {
                Ok(_)   => response.set_status(Status::Ok),
                // TODO write the error to the body in the error case
                Err(_)  => response.set_status(Status::InternalError),
            }
            response
        });
    }
}
