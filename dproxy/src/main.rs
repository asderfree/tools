mod auth;
mod proxy_tls;
mod proxy_http;

use clap::Parser;
use once_cell::sync::Lazy;
#[derive(Debug, Clone, Parser)]
struct Opts {
    #[clap(short, long, required = true)]
    password: String,
}

// Password never changes, so we can use a const.
pub static PROXY_PW: Lazy<&'static str> = Lazy::new(|| {
    let opts = Opts::parse();
    Box::leak(opts.password.into_boxed_str())
});

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry().with(fmt::layer()).with(EnvFilter::from_default_env()).init();

    let (_res1, _res2) = tokio::join!(
        tokio::task::spawn(proxy_tls::main()),
        tokio::task::spawn(proxy_http::main())
    );
    Ok(())
}
