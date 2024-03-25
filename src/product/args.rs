use clap::Parser;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[clap(long, env, default_value_t = SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)
    )]
    pub bind_address: SocketAddr,

    #[clap(long, env, default_value = "info")]
    pub log_level: String,

    #[clap(long, env, default_value = "product_catalog")]
    pub postgres_database: String,

    #[clap(long, env, default_value = "localhost")]
    pub postgres_host: String,

    #[clap(long, env, default_value = "5432")]
    pub postgres_port: u16,

    #[clap(long, env, default_value = "product_catalog")]
    pub postgres_user: String,

    #[clap(long, env, default_value = "product_catalog")]
    pub postgres_password: String,
}
