//! Routing model.

use super::HttpMethod;

#[derive(Debug, Clone)]
pub struct Route {
    pub path: String,
    pub method: HttpMethod,
}

#[derive(Debug, Clone)]
pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router() {
        let mut r = Router::new();
        r.add_route(Route { path: "/".into(), method: crate::HttpMethod::Get });
        assert_eq!(r.routes.len(), 1);
    }
}
