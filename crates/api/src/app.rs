mod domain;
mod handlers;
mod services;

#[cfg(test)]
mod tests;

pub use handlers::build_router;

#[cfg(test)]
pub use crate::auth::build_router_in_memory;
