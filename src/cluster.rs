use crate::config::MulletClusterConfig;
use crate::node::Node;
use crate::state::{MulletState, SharedMulletState};
use slog::debug;
use slog::o;
use slog::Logger;
use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

pub struct Cluster {
    config: MulletClusterConfig,
    logger: Logger,
    state: SharedMulletState,
}

impl Cluster {
    pub fn new(config: MulletClusterConfig, logger: Logger) -> Self {
        let state = Arc::new(Mutex::new(MulletState::new()));
        Cluster {
            config,
            logger,
            state,
        }
    }

    pub fn run(&self) {
        debug!(
            self.logger,
            "Starting Cluster with {} nodes at port range start {}",
            self.config.nodes().len(),
            self.config.low_port()
        );

        for bucket_config in self.config.buckets() {
            self.state
                .lock()
                .expect("could not get lock")
                .borrow_mut()
                .add_bucket(bucket_config.clone());
        }

        let mut port_offset = 0;
        let mut i = 0;
        for node_config in self.config.nodes() {
            debug!(
                self.logger,
                "Starting Node {} at port offset {}", i, port_offset
            );
            let node = Node::new(
                node_config.clone(),
                self.config.low_port() + port_offset,
                self.logger.new(o!()),
                self.state.clone(),
            );
            node.run();
            port_offset = port_offset + 10;
            i = i + 1;
        }
    }
}
