use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MulletClusterConfig {
    nodes: Vec<MulletNodeConfig>,
    low_port: usize,
}

impl MulletClusterConfig {
    pub fn nodes(&self) -> &Vec<MulletNodeConfig> {
        &self.nodes
    }
    pub fn low_port(&self) -> usize {
        self.low_port
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct MulletNodeConfig {
    services: Vec<MulletService>,
}

impl MulletNodeConfig {
    pub fn services(&self) -> &Vec<MulletService> {
        &self.services
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum MulletService {
    #[serde(alias = "query")]
    Query,
}
