use crate::config::MulletClusterConfig;
use crate::node::Node;
use slog::debug;
use slog::o;
use slog::Logger;

pub struct Cluster {
    config: MulletClusterConfig,
    logger: Logger,
}

impl Cluster {
    pub fn new(config: MulletClusterConfig, logger: Logger) -> Self {
        Cluster { config, logger }
    }

    pub fn run(&self) {
        debug!(
            self.logger,
            "Starting Cluster with {} nodes at port range start {}",
            self.config.nodes().len(),
            self.config.low_port()
        );

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
            );
            node.run();
            port_offset = port_offset + 10;
            i = i + 1;
        }
    }
}
