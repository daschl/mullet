pub mod query;
pub mod manager;

pub trait Service {
    fn run(&self);
}
