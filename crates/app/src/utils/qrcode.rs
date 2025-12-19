pub fn create_qrcode<D: AsRef<[u8]>>(data: D) -> anyhow::Result<Vec<u8>> {
  let qrcode_img = qrcode::QrCode::new(data)?
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
