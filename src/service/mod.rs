pub mod kv;
pub mod manager;
pub mod query;

pub trait Service {
    fn run(&self);
}
