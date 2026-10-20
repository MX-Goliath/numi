use clap::Parser;
use env_logger::Env;
use numi::{FormatOptions, NumiModel, NumiModelOptions};
use tokio::runtime::Runtime;

#[derive(Parser, Debug)]
#[command(version = "(dev)")]
#[command(name = "numi-cli")]
#[command(author = "Dzmitry N. <mail@numi.app>")]
struct Cli {
  #[arg(short, long)]
  precision: Option<u32>,
  #[arg(short, long, default_value_t = false)]
  no_cache: bool,
  expression: String,
  #[arg(short, long, default_value_t = false)]
  verbose: bool,
  #[arg(short, long)]
  locale: Option<String>,
}

fn main() -> std::io::Result<()> {
  let cli = Cli::parse();

  if cli.verbose {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
  }

  let mut model_options = NumiModelOptions {
    locale: cli.locale,
    format_options: FormatOptions::default(),
    rates_ttl: 60 * 60,
    locale_ttl: 60 * 60 * 24,
  };

  if cli.no_cache {
    model_options.rates_ttl = 0;
    model_options.locale_ttl = 0;
  }

  if let Some(precision) = cli.precision {
    model_options.format_options.precision = precision;
  }

  let runtime = Runtime::new().expect("Failed to create runtime");
  let rh = runtime.handle();
  let model = NumiModel::new(rh.clone(), model_options, |_u| {});
  let expression = cli.expression.clone();

  runtime.block_on(async move {
    model.start().await;
    let result = model.calc(&expression).await;
    println!("{}", result.unwrap_or("error".to_string()));
  });

  Ok(())
}
