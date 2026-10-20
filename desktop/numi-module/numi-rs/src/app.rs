use crate::{Calc, CalcConfig, FormatOptions, RemoteResourceManager, TmpFileManager};
use log::{debug, error};
use reqwest::Client;
use std::sync::{Arc, Mutex};
use sys_locale::get_locale;
use tokio::runtime::Handle;

pub const REMOTE_RATES_URL: &str = "https://numi.sfo2.cdn.digitaloceanspaces.com/rates/coins2.json";
pub const REMOTE_COINS_URL: &str =
  "https://numi.sfo2.cdn.digitaloceanspaces.com/rates/currency.json";
pub const REMOTE_LOCALE_BASE_PATH: &str = "https://s3.numi.app/locales/";

pub struct NumiModel {
  handle: Handle,
  client: Client,
  options: NumiModelOptions,
  callback: Arc<dyn Fn(NumiModelUpdate) + Send + Sync>,
  remote_rm: Arc<RemoteResourceManager>,
  calc: Arc<Mutex<Option<Calc>>>,
  state: Mutex<NumiModelState>,
}

#[derive(Clone)]
pub struct NumiModelOptions {
  pub locale: Option<String>,
  pub format_options: FormatOptions,
  pub rates_ttl: u64,
  pub locale_ttl: u64,
}

pub enum NumiModelUpdate {
  CalcInitialized,
}

#[derive(Default)]
pub struct NumiModelState {
  pub calc_started: bool,
}

impl NumiModel {
  pub fn new<T>(handle: Handle, options: NumiModelOptions, callback: T) -> Self
  where
    T: Fn(NumiModelUpdate) + Send + Sync + 'static,
  {
    let remote_rm = RemoteResourceManager {
      verbose: false,
      default_cache_sec: options.rates_ttl,
      file_manager: TmpFileManager::default(),
    };

    Self {
      handle,
      client: Client::new(),
      options,
      callback: Arc::new(callback),
      remote_rm: Arc::new(remote_rm),
      calc: Arc::new(Mutex::new(None)),
      state: Mutex::new(NumiModelState::default()),
    }
  }

  pub async fn start(&self) {
    {
      let mut lock = self.state.lock().unwrap();
      {
        let calc_started = &mut lock.calc_started;
        if *calc_started {
          return;
        }
        *calc_started = true;
      }
    }
    let options = self.options.clone();
    let remote_rm = self.remote_rm.clone();
    let calc_mutex = self.calc.clone();
    let callback = self.callback.clone();
    let _ = self
      .handle
      .clone()
      .spawn(async move {
        // Build the English calculator before touching the network. The UI can
        // calculate immediately, and rates/locales are refreshed in the same
        // task when they are available.
        let calc = Calc::with_config(&CalcConfig { locale_path: None });
        {
          let mut calc_guard = calc_mutex.lock().unwrap();
          *calc_guard = Some(calc);
        }
        callback(NumiModelUpdate::CalcInitialized);

        let mut calc_config = CalcConfig { locale_path: None };

        // load locale files
        if let Some(locale) = &options.locale {
          let locale_url = format!("{}{}.strings", REMOTE_LOCALE_BASE_PATH, locale);
          debug!("Passed locale argument, pull from {:}", locale_url);
          if let Some(cache_file) = remote_rm
            .get_url_content_path(&locale_url, Some(options.locale_ttl))
            .await
          {
            calc_config.locale_path = Some(cache_file);
          } else {
            error!("Can't get local file for remote url {:}", locale_url);
          }
        } else {
          let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
          let mut parts = locale.split(|character| character == '-' || character == '_');
          if let Some(lang) = parts.next() {
            if lang != "en" {
              debug!("Trying to pull locale for lang {:}", lang);
              let locale_url = format!("{}{}.strings", REMOTE_LOCALE_BASE_PATH, lang);
              if let Some(cache_file) = remote_rm
                .get_url_content_path(&locale_url, Some(options.locale_ttl))
                .await
              {
                calc_config.locale_path = Some(cache_file);
              }
            }
          }
        }

        // Refresh the calculator with locale and rates when the network responds.
        let mut calc = Calc::with_config(&calc_config);
        if let Some(content) = remote_rm.get_url_content(REMOTE_RATES_URL, None).await {
          calc.fill_rates_from_json_str(&content);
        }
        if let Some(content) = remote_rm.get_url_content(REMOTE_COINS_URL, None).await {
          calc.fill_rates_from_json_str(&content);
        }

        let mut calc_guard = calc_mutex.lock().unwrap();
        *calc_guard = Some(calc);
      })
      .await;
  }

  pub async fn calc(&self, expression: &str) -> Option<String> {
    let handle = self.handle.clone();
    let calc_mutex = self.calc.clone();
    let options = self.options.clone();
    let str = expression.to_owned();
    let result = handle
      .spawn(async move {
        let guard = calc_mutex.lock().unwrap();
        if let Some(ref calc) = *guard {
          return calc.calculate_format(&str, &options.format_options);
        }
        None
      })
      .await;
    result.ok().flatten()
  }

  pub async fn calc_json(&self, input_str: &str) -> Option<String> {
    let handle = self.handle.clone();
    let calc_mutex = self.calc.clone();
    let str = input_str.to_owned();
    let result = handle
      .spawn(async move {
        let guard = calc_mutex.lock().unwrap();
        if let Some(ref calc) = *guard {
          return Some(calc.calc_json(&str));
        }
        None
      })
      .await;
    result.ok().flatten()
  }
}
