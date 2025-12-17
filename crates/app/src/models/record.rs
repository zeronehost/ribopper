use std::path::PathBuf;

use chrono::{DateTime, Local};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Record {
  Text {
    id: u64,
    content: String,
    data: Vec<ribo_clipboard::FormatContent>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
  },
  Image {
    id: u64,
    content: Vec<u8>,
    data: Vec<ribo_clipboard::FormatContent>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
  },
  Files {
    id: u64,
    content: Vec<PathBuf>,
    data: Vec<ribo_clipboard::FormatContent>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
  },
}

impl TryInto<Record> for ribo_db::models::Record {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<Record, Self::Error> {
    let content: ribo_clipboard::FormatContent = serde_json::from_str(&self.content)?;
    let data: Vec<ribo_clipboard::FormatContent> = serde_json::from_str(&self.data)?;

    Ok(match content {
      ribo_clipboard::FormatContent::Text(text) => Record::Text {
        content: text,
        data,
        id: self.id,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
      ribo_clipboard::FormatContent::Image(image) => Record::Image {
        id: self.id,
        content: image,
        data,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
      ribo_clipboard::FormatContent::Files(files) => Record::Files {
        id: self.id,
        content: files,
        data,
        created_at: self.created_at,
        updated_at: self.updated_at,
      },
    })
  }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum RecordWithTargets {
  Text {
    id: u64,
    content: String,
    data: Vec<ribo_clipboard::FormatContent>,
    targets: Vec<String>,
    target_count: usize,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
  },
  Image {
    id: u64,
    content: Vec<u8>,
    data: Vec<ribo_clipboard::FormatContent>,
    targets: Vec<String>,
    target_count: usize,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
  },
  Files {
    id: u64,
    content: Vec<PathBuf>,
    data: Vec<ribo_clipboard::FormatContent>,
    targets: Vec<String>,
    target_count: usize,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
  },
}
impl TryInto<RecordWithTargets> for &ribo_db::models::RecordWithTargets {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<RecordWithTargets, Self::Error> {
    let content: ribo_clipboard::FormatContent = serde_json::from_str(&self.content)?;
    let data: Vec<ribo_clipboard::FormatContent> = serde_json::from_str(&self.data)?;

    Ok(match content {
      ribo_clipboard::FormatContent::Text(text) => RecordWithTargets::Text {
        content: text,
        data,
        id: self.record_id,
        targets: self.target_names.clone(),
        target_count: self.target_count as usize,
        created_at: self.record_created,
        updated_at: self.record_updated,
      },
      ribo_clipboard::FormatContent::Image(image) => RecordWithTargets::Image {
        id: self.record_id,
        content: image,
        data,
        targets: self.target_names.clone(),
        target_count: self.target_count as usize,
        created_at: self.record_created,
        updated_at: self.record_updated,
      },
      ribo_clipboard::FormatContent::Files(files) => RecordWithTargets::Files {
        id: self.record_id,
        content: files,
        data,
        targets: self.target_names.clone(),
        target_count: self.target_count as usize,
        created_at: self.record_created,
        updated_at: self.record_updated,
      },
    })
  }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum UpdateRecord {
  Text { id: u64, content: String },
  Image { id: u64, content: Vec<u8> },
  Files { id: u64, content: Vec<PathBuf> },
}

impl TryInto<(u64, String)> for UpdateRecord {
  type Error = serde_json::Error;
  fn try_into(self) -> Result<(u64, String), Self::Error> {
    match self {
      Self::Text { id, content } => Ok((
        id,
        serde_json::to_string(&ribo_clipboard::FormatContent::Text(content))?,
      )),
      Self::Image { id, content } => Ok((
        id,
        serde_json::to_string(&ribo_clipboard::FormatContent::Image(content))?,
      )),
      Self::Files { id, content } => Ok((
        id,
        serde_json::to_string(&ribo_clipboard::FormatContent::Files(content))?,
      )),
    }
  }
}
