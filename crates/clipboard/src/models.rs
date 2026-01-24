#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RecordType {
  Text,
  #[cfg(feature = "image")]
  Image,
  #[cfg(feature = "file")]
  Files,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Record {
  pub content: String,
  pub data: String,
  #[serde(rename = "type")]
  pub typ: RecordType,
}

#[cfg(feature = "image")]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Image {
  pub width: u32,
  pub height: u32,
  pub data: Vec<u8>,
}
