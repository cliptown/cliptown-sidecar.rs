#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
// f2e-generated module preserves its canonical source formatting and branch
// pattern; keep Clippy warnings denied for all handwritten Rust.
#[allow(clippy::match_like_matches_macro)]
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

const MAX_BIND_BYTES: usize = 128;

/// Reject unbounded or malformed env input without ever printing its contents.
fn sidecar_config_for_bind(value: &str) -> Option<SidecarConfig> {
    if value.len() > MAX_BIND_BYTES {
        return None;
    }
    SidecarConfig::from_bind(SidecarIdentity::new(env::SERVICE, env::BIND), value, false).ok()
}

fn main() {
    let values = env_runtime::load_from_os();
    let cfg = match sidecar_config_for_bind(&values.bind) {
        Some(cfg) => cfg,
        None => {
            eprintln!("invalid CLIPTOWN_SIDECAR_BIND; refusing to start");
            std::process::exit(2);
        }
    };
    runtime::run(&cfg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_bind_configuration_without_reflecting_untrusted_value() {
        assert!(sidecar_config_for_bind(env::BIND_DEFAULT).is_some());
        assert!(sidecar_config_for_bind(&"x".repeat(MAX_BIND_BYTES + 1)).is_none());
        assert!(sidecar_config_for_bind("not a socket address").is_none());
    }

    #[test]
    fn malformed_bind_is_not_a_valid_sidecar_config() {
        let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
        assert!(SidecarConfig::from_bind(identity, "not a socket address", false).is_err());
    }
}
