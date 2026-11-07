//! Middleware model.

#[derive(Debug, Clone)]
pub struct Middleware {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct MiddlewareChain {
    pub middlewares: Vec<Middleware>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self { middlewares: vec![] }
    }

    pub fn add(&mut self, m: Middleware) {
        self.middlewares.push(m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middleware_chain() {
        let mut chain = MiddlewareChain::new();
        chain.add(Middleware { name: "logging".into() });
        assert_eq!(chain.middlewares.len(), 1);
    }
}
