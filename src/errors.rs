use finchers::{Response, Responder};
use finchers::util::NoReturn;
use hyper::StatusCode;

#[derive(Debug)]
pub enum ApiError {
    ParseBody,
}

impl Responder for ApiError {
    type Error = NoReturn;
    fn respond(self) -> Result<Response, Self::Error> {
        use ApiError::*;
        match self {
            ParseBody => Ok(Response::new().with_status(StatusCode::BadRequest)),
        }
    }
}
