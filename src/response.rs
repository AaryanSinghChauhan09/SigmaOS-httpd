//! HTTP response model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok, NotFound, BadRequest, Unauthorized, Forbidden,
    InternalServerError, NotImplemented, ServiceUnavailable,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::Forbidden => 403,
            HttpStatus::InternalServerError => 500,
            HttpStatus::NotImplemented => 501,
            HttpStatus::ServiceUnavailable => 503,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_response() {
        let r = HttpResponse { status: HttpStatus::Ok, headers: vec![], body: vec![] };
        assert_eq!(r.status.code(), 200);
    }
}
