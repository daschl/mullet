use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MulletClusterConfig {
    nodes: Vec<MulletNodeConfig>,
    buckets: Vec<MulletBucketConfig>,
    low_port: usize,
}

impl MulletClusterConfig {
    pub fn nodes(&self) -> &Vec<MulletNodeConfig> {
        &self.nodes
    }
    pub fn low_port(&self) -> usize {
        self.low_port
    }
    pub fn buckets(&self) -> &Vec<MulletBucketConfig> {
        &self.buckets
    }
}

impl Default for MulletClusterConfig {
    fn default() -> Self {
        MulletClusterConfig {
            nodes: vec![MulletNodeConfig {
                services: vec![MulletService::KeyValue, MulletService::Query],
            }],
            buckets: vec![MulletBucketConfig {
                name: "dye-hard".into(),
                ty: MulletBucketType::Couchbase,
            }],
            low_port: 9000,
        }
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
    #[serde(alias = "kv")]
    KeyValue,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MulletBucketConfig {
    name: String,
    #[serde(alias = "type")]
    ty: MulletBucketType,
}

impl MulletBucketConfig {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn ty(&self) -> &MulletBucketType {
        &self.ty
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum MulletBucketType {
    #[serde(alias = "couchbase")]
    Couchbase,
}
