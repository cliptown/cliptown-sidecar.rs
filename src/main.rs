#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

fn main() {
    let values = env_runtime::load_from_os();
    let cfg = match SidecarConfig::from_bind(
        SidecarIdentity::new(env::SERVICE, env::BIND),
        &values.bind,
        false,
    ) {
        Ok(cfg) => cfg,
        Err(_) => {
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
    fn malformed_bind_is_not_a_valid_sidecar_config() {
        let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
        assert!(SidecarConfig::from_bind(identity, "not a socket address", false).is_err());
    }
}
