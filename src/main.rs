use anyhow::Result;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    info!("Hello, Mom!");

    Ok(())
}
