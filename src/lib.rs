//! SigmaOS-httpd — HTTP server model.
//!
//! Models HTTP request handling, responses, routes, middleware,
//! and server lifecycle. Deterministic and std-only.

pub mod server;
pub mod request;
pub mod response;
pub mod route;
pub mod middleware;

pub use server::{HttpServer, ServerConfig};
pub use request::{HttpRequest, HttpMethod, HttpVersion};
pub use response::{HttpResponse, HttpStatus};
pub use route::{Route, Router};
pub use middleware::{Middleware, MiddlewareChain};
