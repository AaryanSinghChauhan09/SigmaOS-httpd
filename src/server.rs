//! HTTP server model.

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { host: "0.0.0.0".into(), port: 8080, max_connections: 100 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerState { Stopped, Starting, Running, Stopping }

#[derive(Debug, Clone)]
pub struct HttpServer {
    pub config: ServerConfig,
    pub state: ServerState,
}

impl HttpServer {
    pub fn new(config: ServerConfig) -> Self {
        Self { config, state: ServerState::Stopped }
    }

    pub fn start(&mut self) {
        self.state = ServerState::Running;
    }

    pub fn stop(&mut self) {
        self.state = ServerState::Stopped;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_server() {
        let mut s = HttpServer::new(ServerConfig::default());
        s.start();
        assert_eq!(s.state, ServerState::Running);
    }
}
