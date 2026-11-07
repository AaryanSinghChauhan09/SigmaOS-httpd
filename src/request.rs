//! HTTP request model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod { Get, Post, Put, Delete, Head, Options, Patch }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpVersion { Http10, Http11, Http2 }

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub version: HttpVersion,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_request() {
        let r = HttpRequest { method: HttpMethod::Get, version: HttpVersion::Http11, path: "/".into(), headers: vec![], body: vec![] };
        assert_eq!(r.method, HttpMethod::Get);
    }
}
