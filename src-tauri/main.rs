// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::{Args, Parser, Subcommand};
use huehuehue::{
    bindings, core::handlers::*, huehuehue_handlers, HueHueHue, HueHueHueBackendConfig,
    HueHueHueBackendError, HueHueHueState,
};
use log::info;
use tauri::RunEvent;
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum HueHueHueError {
    #[error(transparent)]
    BackendError(#[from] HueHueHueBackendError),
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct HueHueHueConfig {
    #[command(subcommand)]
    command: Option<Command>,
}

impl From<&HueHueHueConfig> for HueHueHueBackendConfig {
    fn from(_val: &HueHueHueConfig) -> Self {
        HueHueHueBackendConfig::default()
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Discover(DiscoveryConfig),
    GenerateBindings(BindingsConfig),
}

#[derive(Debug, Args)]
struct DiscoveryConfig {}

impl From<&DiscoveryConfig> for HueHueHueBackendConfig {
    fn from(_val: &DiscoveryConfig) -> Self {
        HueHueHueBackendConfig::default()
    }
}

#[derive(Debug, Args)]
struct BindingsConfig {
    #[arg(short = 'O', long)]
    only: bool,
    #[arg(short = 'o', long, default_value = "../src/bindings.ts")]
    output: String,
}

/// returns `true` if the executable should exit after running the command
async fn command(command: &Command) -> Result<bool, HueHueHueError> {
    info!("running command config: {:?}", command);
    match command {
        Command::Discover(config) => {
            HueHueHue::with_config(config)
                .discover()
                .await
                .map_err(Into::<HueHueHueBackendError>::into)??;

            return Ok(true);
        }
        Command::GenerateBindings(config) => {
            let path = config.output.as_str();
            info!("generating bindings to \"{}\"", path);
            bindings!(path);
            info!("successfully generated bindings to \"{}\"", path);

            if config.only {
                return Ok(true);
            }
        }
    };

    Ok(false)
}

#[tokio::main]
pub async fn main() -> Result<(), HueHueHueError> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let config = HueHueHueConfig::parse();
    info!("run config: {:?}", config);
    if let Some(com) = &config.command {
        if command(com).await? {
            return Ok(());
        }
    }
    let mut huehuehue: HueHueHue = HueHueHue::with_config(&config);
    info!("starting huehuehue...");
    huehuehue.discover();
    huehuehue_handlers!(tauri::Builder::default())
        .manage(HueHueHueState(Mutex::new(huehuehue)))
        .build(tauri::generate_context!())
        .expect("error building tauri app")
        .run(|_, event| match event {
            RunEvent::ExitRequested { .. } => info!("exit requested"),
            RunEvent::Exit => info!("gracefully exiting huehuehue..."),
            _ => (),
        });

    Ok(())
}
