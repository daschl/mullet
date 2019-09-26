use crate::config::{MulletNodeConfig, MulletService};
use crate::service::manager::ManagerService;
use crate::service::query::QueryService;
use crate::service::Service;
use crate::state::SharedMulletState;
use slog::debug;
use slog::o;
use slog::Logger;
use crate::service::kv::KeyValueService;

pub struct Node {
    config: MulletNodeConfig,
    port_base: usize,
    logger: Logger,
    state: SharedMulletState,
}

impl Node {
    pub fn new(
        config: MulletNodeConfig,
        port_base: usize,
        logger: Logger,
        state: SharedMulletState,
    ) -> Self {
        Node {
            config,
            port_base,
            logger,
            state,
        }
    }

    pub fn run(&self) {
        let mgr_port = self.port_base + 1;
        debug!(self.logger, "Starting Manager Service at port {}", mgr_port);
        let mgr_service = ManagerService::new(mgr_port, self.logger.new(o!()), self.state.clone());
        mgr_service.run();

        for service in self.config.services() {
            match service {
                MulletService::Query => {
                    let port = self.port_base + 3;
                    debug!(self.logger, "Starting Query Service at port {}", port);
                    let service =
                        QueryService::new(port, self.logger.new(o!()), self.state.clone());
                    service.run();
                },
                MulletService::KeyValue => {
                    let port = self.port_base + 2;
                    debug!(self.logger, "Starting KV Service at port {}", port);
                    let service =
                        KeyValueService::new(port, self.logger.new(o!()), self.state.clone());
                    service.run();
                }
            }
        }
    }
}
