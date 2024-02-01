use std::{ffi::OsStr, fmt::Display};

use anyhow::Context;

/// Obtains an environment variable that is required by
/// the app, if the environment variable is missing an
/// error is returned.
pub fn require_env<K>(key: K) -> anyhow::Result<String>
where
    K: AsRef<OsStr> + Display,
{
    std::env::var(&key)
        // Append the details as a context message
        .with_context(|| format!("Missing {key} environment variable"))
}

/// Obtains an environment variable `key` that is required by
/// the app, if the environment variable is missing an error
/// is returned.
///
/// Where the `key` is prefixed by `prefix`
pub fn require_env_prefixed<P, K>(prefix: P, key: K) -> anyhow::Result<String>
where
    P: Display,
    K: Display,
{
    let prefixed = format!("{}_{}", prefix, key);

    std::env::var(&prefixed)
        // Append the details as a context message
        .with_context(|| format!("Missing {prefixed} environment variable"))
}
