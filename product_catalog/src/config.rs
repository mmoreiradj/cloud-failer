use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tracing::Level;

#[derive(Parser, Debug, Clone)]
pub struct ProductCatalogConfig {
    #[clap(long, env, default_value_t = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3001)
    )]
    pub bind_address: SocketAddr,

    #[clap(long, env, default_value = Level::INFO.as_str())]
    pub log_level: Level,
}
