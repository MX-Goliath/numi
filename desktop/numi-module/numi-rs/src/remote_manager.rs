use std::fs::{self, metadata, read_to_string};
use std::io::prelude::*;
use std::time::SystemTime;
use std::{env::temp_dir, fs::File};

pub struct RemoteResourceManager {
  pub verbose: bool,
  pub default_cache_sec: u64,
  // TODO: use tmp file manager
  pub file_manager: TmpFileManager,
}

impl Default for RemoteResourceManager {
  fn default() -> Self {
    Self {
      verbose: false,
      default_cache_sec: 60 * 60,
      file_manager: TmpFileManager::default(),
    }
  }
}

impl RemoteResourceManager {
  pub async fn get_url_content_path(&self, url: &str, ttl: Option<u64>) -> Option<String> {
    if self.get_url_content(url, ttl).await.is_some() {
      return Some(self.get_url_cache_path(url));
    }
    None
  }

  pub fn get_url_content_cached_path(&self, url: &str, ttl: Option<u64>) -> Option<String> {
    if self.get_url_content_cached(url, ttl).is_some() {
      return Some(self.get_url_cache_path(url));
    }
    None
  }

  pub async fn get_url_content(&self, url: &str, ttl: Option<u64>) -> Option<String> {
    if let Some(content) = self.get_url_content_cached(url, ttl) {
      return Some(content);
    }

    let file_path = self.get_url_cache_path(url);
    // Explicitly remove so no cache if failed to pull from the web
    let _ = fs::remove_file(&file_path);

    let client = reqwest::Client::builder()
      .connect_timeout(std::time::Duration::from_secs(3))
      .timeout(std::time::Duration::from_secs(8))
      .build()
      .ok()?;
    let resp = client.get(url).send().await;
    // dbg!(&resp);
    if let Ok(resp) = resp {
      if let Ok(text) = resp.text().await {
        if let Ok(mut f) = File::create(file_path) {
          let _ = f.write_all(text.as_bytes());
        }
        return Some(text);
      }
    }

    None
  }

  pub fn get_url_content_cached(&self, url: &str, ttl: Option<u64>) -> Option<String> {
    let file_path = self.get_url_cache_path(url);
    if let Ok(metadata) = metadata(&file_path) {
      if let Ok(mtime) = metadata.modified() {
        let diff = SystemTime::now().duration_since(mtime);
        if let Ok(duration) = diff {
          let used_ttl = ttl.unwrap_or(self.default_cache_sec);
          if duration.as_secs() < used_ttl {
            if let Ok(content) = read_to_string(&file_path) {
              if self.verbose {
                println!("Load cached version for path {}", url);
              }
              return Some(content);
            }
          }
        }
      }
    }
    None
  }

  pub fn get_url_cache_path(&self, remote_path: &str) -> String {
    let file_name = remote_path.split('/').last().unwrap();
    let file_path = format!("{}/{}", temp_dir().display(), file_name);
    file_path
  }
}

#[derive(Default)]
pub struct TmpFileManager {}

impl TmpFileManager {
  pub fn file_path(&self, file_name: &str) -> String {
    let file_path = format!("{}/{}", temp_dir().display(), file_name);
    file_path
  }

  pub fn write_file(&self, file_name: &str, contents: &str) {
    let file_path = self.file_path(file_name);
    if let Ok(mut f) = File::create(file_path) {
      let _ = f.write_all(contents.as_bytes());
    }
  }

  pub fn read_file(&self, file_name: &str) -> Result<String, std::io::Error> {
    let file_path = self.file_path(file_name);
    read_to_string(file_path)
  }
}
