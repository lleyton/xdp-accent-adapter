use crate::{color::Color, config::Config};

mod adw_gtk3;
mod libadwaita;

#[tracing::instrument]
pub async fn update(config: &Config, color: &Color) -> color_eyre::Result<()> {
    if config.backends.libadwaita.enabled {
        tracing::info!("Updating libadwaita color");
        libadwaita::update(color).await?;
    }

    if config.backends.adw_gtk3.enabled {
        tracing::info!("Updating adw-gtk3 color");
        adw_gtk3::update(color).await?;
    }

    Ok(())
}
