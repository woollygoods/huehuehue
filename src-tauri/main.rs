// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use huehuehue::core::endpoints::*;
use huehuehue::huehuehue_handlers;
use huehuehue::HueHueHue;
use huehuehue::HueHueHueError;
use huehuehue::HueHueHueState;
use log::info;
use tauri::RunEvent;
use tokio::sync::Mutex;
use huehuehue::bindings;

#[tokio::main]
pub async fn main() -> Result<(), HueHueHueError> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    info!("starting huehuehue...");
    let huehuehue: HueHueHue = HueHueHue::default();
    huehuehue.discover()?;
    bindings!("../../src/bindings.ts");
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
