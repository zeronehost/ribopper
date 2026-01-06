use std::path::PathBuf;

use chrono::{DateTime, Local};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Record {
  id: u64,
  pub(crate) text: Option<String>,
  pub(crate) image: Option<Vec<u8>>,
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
        image: None,
        files: None,
        typ: self.typ,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
      ribo_db::models::RecordType::Image => Record {
        id: self.id,
        text: None,
        image: Some(serde_json::from_str(&self.content)?),
        files: None,
        typ: self.typ,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
      ribo_db::models::RecordType::Files => Record {
        id: self.id,
        text: None,
        image: None,
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
// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct RecordWithTargets {
//   id: u64,
//   text: Option<String>,
//   image: Option<Vec<u8>>,
//   files: Option<Vec<PathBuf>>,
//   #[serde(rename = "type")]
//   typ: ribo_db::models::RecordType,
//   targets: Vec<String>,
//   target_count: u64,
//   created_at: Option<DateTime<Local>>,
//   updated_at: Option<DateTime<Local>>,
// }
// impl TryInto<RecordWithTargets> for &ribo_db::models::RecordWithTargets {
//   type Error = serde_json::Error;
//   fn try_into(self) -> Result<RecordWithTargets, Self::Error> {
//     Ok(match self.record_type {
//       ribo_db::models::RecordType::Text => RecordWithTargets {
//         id: self.record_id,
//         text: Some(self.content.clone()),
//         image: None,
//         files: None,
//         typ: self.record_type,
//         targets: self.target_names.clone(),
//         target_count: self.target_count,
//         created_at: self.record_created,
//         updated_at: self.record_updated,
//       },
//       ribo_db::models::RecordType::Image => RecordWithTargets {
//         id: self.record_id,
//         image: Some(serde_json::from_str(&self.content)?),
//         text: None,
//         files: None,
//         typ: self.record_type,
//         targets: self.target_names.clone(),
//         target_count: self.target_count,
//         created_at: self.record_created,
//         updated_at: self.record_updated,
//       },
//       ribo_db::models::RecordType::Files => RecordWithTargets {
//         id: self.record_id,
//         files: Some(serde_json::from_str(&self.content)?),
//         image: None,
//         text: None,
//         typ: self.record_type,
//         targets: self.target_names.clone(),
//         target_count: self.target_count,
//         created_at: self.record_created,
//         updated_at: self.record_updated,
//       },
//     })
//   }
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdateRecord {
  id: u64,
  text: Option<String>,
  image: Option<Vec<u8>>,
  files: Option<Vec<PathBuf>>,
  #[serde(rename = "type")]
  pub(crate) typ: ribo_db::models::RecordType,
}

impl TryInto<(u64, String)> for UpdateRecord {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<(u64, String), Self::Error> {
    match self.typ {
      ribo_db::models::RecordType::Text => Ok((self.id, self.text.unwrap_or_default())),
      ribo_db::models::RecordType::Image => Ok((
        self.id,
        serde_json::to_string(&self.image.unwrap_or_default())?,
      )),
      ribo_db::models::RecordType::Files => Ok((
        self.id,
        serde_json::to_string(&self.files.unwrap_or_default())?,
      )),
    }
  }
}
