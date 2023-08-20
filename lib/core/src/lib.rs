pub use tracing::{debug, error, info, trace, warn};

use tracing_subscriber::EnvFilter;

/// A function for initializing the core tracing pattern
pub fn init_tracing() {
    // Initialize environment logger for all macros to use
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}