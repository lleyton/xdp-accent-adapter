use self::config::Config;
use crate::color::Color;
use ashpd::desktop::settings::Settings;
use futures_util::StreamExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

mod backends;
mod color;
mod config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let filter: EnvFilter = EnvFilter::try_from_env("XDP_ACCENT_ADAPTER_LOG")
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = fmt::layer().pretty().with_filter(filter);
    let subscriber = Registry::default()
        .with(tracing_error::ErrorLayer::default())
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config = Config::load().await?.unwrap_or_default();

    let settings = Settings::new().await?;
    let color = settings
        .read::<color::Color>("org.freedesktop.appearance", "accent-color")
        .await?;

    backends::update(&config, &color).await?;

    let mut changed = settings
        .receive_setting_changed_with_args("org.freedesktop.appearance", "accent-color")
        .await?;

    while let Some(item) = changed.next().await {
        let color: Color = item.value().to_owned().try_into()?;
        backends::update(&config, &color).await?;
    }

    Ok(())
}
