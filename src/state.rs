use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use serde_json::json;
use crate::config::{MulletBucketConfig, MulletBucketType};

pub type SharedMulletState = Arc<Mutex<MulletState>>;

#[derive(Debug)]
pub struct MulletState {
    buckets: HashMap<String, BucketState>
}

impl MulletState {
    pub fn new() -> Self {
        Self { buckets: HashMap::new() }
    }

    pub fn add_bucket(&mut self, spec: MulletBucketConfig) {
        let name = spec.name();

        let ty = match spec.ty() {
            MulletBucketType::Couchbase => "membase",
        };

        if !self.buckets.contains_key(name) {
            self.buckets.insert(name.clone(), BucketState::new(name.clone(), ty.into()));
        }
    }

    pub fn export_bucket_config(&self, name: &String) -> Option<serde_json::Value> {
        self.buckets.get(name).map(|bc| {
            json!({
                "name": bc.name,
                "bucketType": bc.ty,
            })
        })
    }

    pub fn export_all_bucket_configs(&self) -> Vec<serde_json::Value> {
        self.buckets.keys().map(|k| self.export_bucket_config(k).unwrap()).collect()
    }
}

#[derive(Debug)]
pub struct BucketState {
    name: String,
    ty: String,
}

impl BucketState {
    pub fn new(name: String, ty: String) -> Self {
        Self { name, ty }
    }
}