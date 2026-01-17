use std::path::PathBuf;

use chrono::{DateTime, Local};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Record {
  id: u64,
  pub(crate) text: Option<String>,
  #[cfg(feature = "image")]
  pub(crate) image: Option<Vec<u8>>,
  #[cfg(feature = "file")]
  pub(crate) files: Option<Vec<PathBuf>>,
  #[serde(rename = "type")]
  pub(crate) typ: ribo_db::models::RecordType,
  created_at: DateTime<Local>,
  updated_at: DateTime<Local>,
}

impl TryInto<Record> for ribo_db::models::Record {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<Record, Self::Error> {
    Ok(match self.typ {
      ribo_db::models::RecordType::Text => Record {
        id: self.id,
        text: Some(self.content.clone()),
        #[cfg(feature = "image")]
        image: None,
        #[cfg(feature = "file")]
        files: None,
        typ: self.typ,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
      #[cfg(feature = "image")]
      ribo_db::models::RecordType::Image => Record {
        id: self.id,
        text: None,
        #[cfg(feature = "image")]
        image: Some(serde_json::from_str(&self.content)?),
        #[cfg(feature = "file")]
        files: None,
        typ: self.typ,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
      #[cfg(feature = "file")]
      ribo_db::models::RecordType::Files => Record {
        id: self.id,
        text: None,
        #[cfg(feature = "image")]
        image: None,
        #[cfg(feature = "file")]
        files: Some(serde_json::from_str(&self.content)?),
        typ: self.typ,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
    })
  }
}

impl TryInto<Record> for &ribo_db::models::Record {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<Record, Self::Error> {
    self.clone().try_into()
  }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateRecord {
  id: u64,
  text: Option<String>,
  #[cfg(feature = "image")]
  image: Option<Vec<u8>>,
  #[cfg(feature = "file")]
  files: Option<Vec<PathBuf>>,
  #[serde(rename = "type")]
  pub(crate) typ: ribo_db::models::RecordType,
}

impl TryInto<(u64, String)> for UpdateRecord {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<(u64, String), Self::Error> {
    match self.typ {
      ribo_db::models::RecordType::Text => Ok((self.id, self.text.unwrap_or_default())),
      #[cfg(feature = "image")]
      ribo_db::models::RecordType::Image => Ok((
        self.id,
        serde_json::to_string(&self.image.unwrap_or_default())?,
      )),
      #[cfg(feature = "file")]
      ribo_db::models::RecordType::Files => Ok((
        self.id,
        serde_json::to_string(&self.files.unwrap_or_default())?,
      )),
    }
  }
}
