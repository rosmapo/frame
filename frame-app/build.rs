use std::{env, path::Path};

const LINUX_APP_ICON: &str = "resources/app-icons/icon.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={LINUX_APP_ICON}");

    prepare_app_icon_x11()
}

fn prepare_app_icon_x11() -> Result<(), Box<dyn std::error::Error>> {
    use image::{ImageReader, imageops};

    let icon = ImageReader::open(LINUX_APP_ICON)?
        .decode()?
        .resize(256, 256, imageops::FilterType::Lanczos3)
        .into_rgba8();
    let icon_out_path = Path::new(&env::var("OUT_DIR")?).join("app_icon.png");
    icon.save(icon_out_path)?;

    Ok(())
}
