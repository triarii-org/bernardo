mod promise;
pub use promise::{Promise, PromiseState, ResolvedPromise, UpdateResult};

mod promise_map;
pub use promise_map::MappedPromise;

// TODO(XXX): remove
mod promise_shared;
