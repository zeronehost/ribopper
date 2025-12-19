use clipboard_master::{CallbackResult, ClipboardHandler, Master};
use std::{
  path::PathBuf,
  sync::{Arc, Mutex, mpsc},
  thread,
};

pub struct Manager<F> {
  inner: Arc<Mutex<InnerManager<F>>>,
}

impl<F> Manager<F>
where
  F: Fn(Content) + Send + 'static,
{
  pub fn new(handler: F) -> crate::error::Result<Self> {
    let (tx, rx) = mpsc::channel();
    let manager = Self {
      inner: Arc::new(Mutex::new(InnerManager {
        handler: Box::new(handler),
        clipboard: crate::clipboard::Clipboard::new()?,
        flag: false,
      })),
    };

    manager.listen(rx, tx);
    Ok(manager)
  }

  fn listen(&self, rx: mpsc::Receiver<()>, tx: mpsc::Sender<()>) {
    thread::spawn(move || {
      Master::new(Handler(tx)).unwrap().run().unwrap();
    });

    let inner = self.inner.clone();
    thread::spawn(move || {
      for _ in rx {
        let mut inner = inner.lock().unwrap();
        if let Some(content) = inner.get_content() {
          (*inner.handler)(content);
        }
      }
    });
  }

  pub fn paste(&self, content: Content) -> crate::error::Result<()> {
    self.inner.lock().unwrap().paste(content)
  }
}

struct Handler(mpsc::Sender<()>);

impl ClipboardHandler for Handler {
  fn on_clipboard_change(&mut self) -> CallbackResult {
    if let Err(e) = self.0.send(()) {
      log::error!("failed to send clipboard change event {e}");
    }
    CallbackResult::Next
  }
  fn on_clipboard_error(&mut self, _error: std::io::Error) -> CallbackResult {
    CallbackResult::Next
  }
}

struct InnerManager<F> {
  handler: Box<F>,
  clipboard: crate::clipboard::Clipboard,
  flag: bool,
}

impl<F> InnerManager<F>
where
  F: Fn(Content) + Send + 'static,
{
  fn get_content(&mut self) -> Option<Content> {
    if self.flag {
      self.flag = false;
      return None;
    }
    let mut data: Vec<FormatContent> = vec![];
    let mut content = None;

    if let Ok(text) = self.clipboard.get_text() {
      content = Some(FormatContent::Text(text.clone()));
      data.push(FormatContent::Text(text));
    }
    if let Ok(png) = self.clipboard.get_image() {
      content = Some(FormatContent::Image(png.clone()));
      data.push(FormatContent::Image(png));
    }

    if let Ok(files) = self.clipboard.get_files() {
      content = Some(FormatContent::Files(files.clone()));
      data.push(FormatContent::Files(files));
    }

    if content.is_none() || data.is_empty() {
      return None;
    }

    Some(Content {
      content: content.unwrap(),
      data,
    })
  }

  fn paste(&mut self, content: Content) -> crate::error::Result<()> {
    for data in content.data {
      match data {
        FormatContent::Text(data) => {
          self.clipboard.set_text(data.as_str())?;
        }
        FormatContent::Image(data) => {
          self.clipboard.set_image(data.as_slice())?;
        }
        FormatContent::Files(data) => {
          self.clipboard.set_files(data.as_slice())?;
        }
      }
    }
    self.flag = true;
    Ok(())
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
  pub content: FormatContent,
  pub data: Vec<FormatContent>,
}

impl Content {
  fn get_type(&self) -> ribo_db::models::RecordType {
    self.content.get_type()
  }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FormatContent {
  Text(String),
  Image(Vec<u8>),
  Files(Vec<PathBuf>),
}

impl FormatContent {
  fn get_type(&self) -> ribo_db::models::RecordType {
    match self {
      Self::Text(_) => ribo_db::models::RecordType::Text,
      Self::Image(_) => ribo_db::models::RecordType::Image,
      Self::Files(_) => ribo_db::models::RecordType::Files,
    }
  }
}

impl TryFrom<Content> for ribo_db::models::NewRecord {
  type Error = serde_json::Error;
  fn try_from(value: Content) -> Result<Self, Self::Error> {
    let typ = value.get_type();
    let content = match value.content {
      FormatContent::Text(data) => data,
      FormatContent::Image(data) => serde_json::to_string(&data)?,
      FormatContent::Files(data) => serde_json::to_string(&data)?,
    };
    Ok(ribo_db::models::NewRecord {
      content,
      data: serde_json::to_string(&value.data)?,
      typ,
    })
  }
}
