pub mod manager;
pub mod query;
pub mod kv;

pub trait Service {
    fn run(&self);
}
