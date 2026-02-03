use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "chml")]
#[command(
    version = concat!(env!("CARGO_PKG_VERSION"), " by ", env!("CARGO_PKG_AUTHORS"))
)]
#[command(author = env!("CARGO_PKG_AUTHORS"),about = "A cli tool for Chmlfrp; https://www.chmlfrp.net", long_about = None)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub quiet: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// list tunnels, domains, nodes
    Ls {
        #[arg(short = 't', long)]
        tunnels: bool,
        #[arg(short = 'd', long)]
        domains: bool,
        /// list all nodes include RTT <chml ping>
        #[arg(short = 'n', long)]
        nodes: bool,
        #[arg(short = 'c', long)]
        configs: bool,
    },
    /// connect a tunnel
    Connect {
        #[arg(short = 't', long)]
        tunnel: Option<String>,
        #[arg(short = 'i', long)]
        tunnel_id: Option<u64>,
        #[arg(short, long)]
        daemon: bool, // todo
    },
    /// add tunnel, domain
    Add {
        #[command(subcommand)]
        resource: Add,
    },
    /// remove tunnel, domain
    Rm {
        #[arg(long)]
        tunnel_id: Option<String>,
        #[arg(short = 't', long)]
        tunnel: Option<String>,
        // #[arg(short = 'd', long)]
        // domain: Option<String>,
    },
    /// get tunnel config
    Get {
        #[arg(short = 't', long)]
        tunnel: String,
    },
    /// quick tcp tunnel
    Tcp { port: u16 },
    /// quick udp tunnel
    Udp { port: u16 },
    /// quick http tunnel
    Http { port: u16 },
    /// print and rebuild node RTT cache (sudo permission required)
    Ping,
}

#[derive(Subcommand)]
pub enum Add {
    /// Add a tunnel
    Tunnel {
        #[arg(long)]
        name: Option<String>,
        #[arg(
            short,
            long,
            value_parser = ["tcp", "udp", "http"]
        )]
        r#type: String,
        #[arg(short, long)]
        lhost: Option<String>,
        #[arg(long)]
        lport: u16,
        #[arg(long)]
        rport: Option<u16>,
        #[arg(short, long)]
        node: Option<String>,
        #[arg(short, long, default_value_t = true)]
        china: bool,
    },
    // Add a domain
    // Domain {
    //     #[arg(short, long)]
    //     record: String,
    //     #[arg(long, default_value = "A")]
    //     r#type: String,
    //     #[arg(long)]
    //     ttl: Option<String>,
    //     #[arg(long)]
    //     rhost: Option<String>,
    // },
}
