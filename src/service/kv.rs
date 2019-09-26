use crate::service::Service;
use crate::state::SharedMulletState;
use actix_server::{Io, Server};
use actix_service::{service_fn, NewService};
use futures::future;
use slog::Logger;

pub struct KeyValueService {
    port: usize,
    logger: Logger,
    state: SharedMulletState,
}

impl KeyValueService {
    pub fn new(port: usize, logger: Logger, state: SharedMulletState) -> Self {
        KeyValueService {
            port,
            logger,
            state,
        }
    }
}

impl Service for KeyValueService {
    fn run(&self) {
        let state = self.state.clone();

        Server::build()
            .bind("kv", format!("127.0.0.1:{}", self.port), || {
                service_fn(move |stream: Io<tokio_tcp::TcpStream>| future::ok::<(), ()>(()))
            })
            .expect("Could not start kv socket")
            .start();
    }
}
