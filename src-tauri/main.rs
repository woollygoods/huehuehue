// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use huehuehue::HueHueHueState;
use log::info;
use tauri::{Manager, RunEvent};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    info!("starting huehuehue...");
    tauri::Builder::default()
        .manage(HueHueHueState(Default::default()))
        .setup(|app| {
            let state = app.state::<HueHueHueState>();
            state.0.lock().unwrap().discover().unwrap();

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error building tauri app")
        .run(|_, event| match event {
            RunEvent::ExitRequested { .. } => info!("exit requested"),
            RunEvent::Exit => info!("gracefully exiting huehuehue..."),
            _ => (),
        });

    Ok(())
}
