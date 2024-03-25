use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::Level;

#[derive(Parser, Debug, Clone)]
pub struct FrontendConfig {
    #[clap(long, env, default_value_t = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)
    )]
    pub bind_address: SocketAddr,

    #[clap(long, env, default_value = Level::INFO.as_str())]
    pub log_level: Level,

    #[clap(long, env, default_value = "http://localhost:3001")]
    pub product_catalog_uri: String,

    #[clap(long, env, default_value = "frontend/static")]
    pub static_dir: String,
}
