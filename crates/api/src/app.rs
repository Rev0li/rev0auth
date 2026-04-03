mod domain;
mod handlers;
mod services;

#[cfg(test)]
mod tests;

pub use handlers::build_router;
