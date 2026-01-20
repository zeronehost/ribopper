use tauri::{AppHandle, Runtime};

use super::error::Result;

pub fn create_qrcode<R: Runtime>(
  data: crate::models::Record,
  #[cfg(feature = "image")]
  app: &AppHandle<R>,
) -> Result<Vec<u8>> {
  let binary = match data.typ {
    ribo_db::models::RecordType::Text => match data.text {
      Some(text) => text.as_bytes().to_vec(),
      None => return Err(anyhow::anyhow!("No text provided").into()),
    },
    #[cfg(feature = "image")]
    ribo_db::models::RecordType::Image => match data.image {
      Some(image) => {
        use crate::utils::path::get_images_path;
        let p = get_images_path(app)?.join(image);
        let image = image::open(p)?;
        let mut buffer = Vec::new();
        image.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageFormat::Png)?;
        buffer
      },
      None => return Err(anyhow::anyhow!("No text provided").into()),
    },
    #[cfg(feature = "file")]
    ribo_db::models::RecordType::Files => match data.files {
      Some(files) => files
        .iter()
        .map(|file| file.display().to_string())
        .collect::<Vec<String>>()
        .join("\n")
        .as_bytes()
        .to_vec(),
      None => return Err(anyhow::anyhow!("No files provided").into()),
    },
  };
  let qrcode_img = qrcode::QrCode::new(binary.as_slice())?
    .render::<image::Luma<u8>>()
    // .max_dimensions(400, 400)
    .min_dimensions(300, 300)
    .quiet_zone(true)
    .build();
  let mut buffer = Vec::new();
  qrcode_img.write_to(
    &mut std::io::Cursor::new(&mut buffer),
    image::ImageFormat::Png,
  )?;

  Ok(buffer)
}
