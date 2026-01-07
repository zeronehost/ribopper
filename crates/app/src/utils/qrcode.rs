pub fn create_qrcode(data: crate::models::Record) -> anyhow::Result<Vec<u8>> {
  let binary = match data.typ {
    ribo_db::models::RecordType::Text => match data.text {
      Some(text) => text.as_bytes().to_vec(),
      None => return Err(anyhow::anyhow!("No text provided")),
    },
    #[cfg(feature = "image")]
    ribo_db::models::RecordType::Image => match data.image {
      Some(image) => image,
      None => return Err(anyhow::anyhow!("No image provided")),
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
      None => return Err(anyhow::anyhow!("No files provided")),
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
