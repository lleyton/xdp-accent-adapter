use tokio::fs;

use crate::color::Color;

pub async fn update(color: &Color) -> color_eyre::Result<()> {
    let base_directories = xdg::BaseDirectories::new()?;
    let path = base_directories.place_config_file("gtk-4.0/gtk.css")?;

    fs::write(
        path,
        format!("@define-color accent_color {};\n", color.to_hex_code()),
    )
    .await?;

    Ok(())
}
