// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use huehuehue::{
    bindings, core::handlers::*, huehuehue_handlers, HueHueHue, HueHueHueError, HueHueHueState,
};
use log::info;
use tauri::RunEvent;
use tokio::sync::Mutex;

const TS_BINDINGS_FILE: &str = "../src/bindings.ts";

#[tokio::main]
pub async fn main() -> Result<(), HueHueHueError> {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    let huehuehue: HueHueHue = HueHueHue::default();
    info!("run config: {:?}", huehuehue.get_config());
    if huehuehue.get_config().generate_bindings_only {
        info!("generating bindings to \"{}\"", TS_BINDINGS_FILE);
        bindings!(TS_BINDINGS_FILE);
        info!(
            "successfully generated bindings to \"{}\"",
            TS_BINDINGS_FILE
        );

        return Ok(());
    }
    info!("starting huehuehue...");
    huehuehue.discover()?;
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
