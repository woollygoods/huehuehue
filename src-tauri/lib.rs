#![feature(macro_metavar_expr)]
pub mod core;

use futures_util::{pin_mut, stream::StreamExt};
use log::info;
use mdns::RecordKind;
use reachable::*;
use reqwest::Client;
use std::{collections::HashMap, net::IpAddr, sync::Arc, time::Duration};
use tokio::{
    sync::Mutex,
    task::{AbortHandle, JoinHandle},
};

pub const HUE_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";
const HUE_BRIDGE_SERVICE_QUERY_INTERVAL_SECONDS: u64 = 1;
const HUE_BRIDGE_API_SCHEMA: &str = "https://";
const HUE_BRIDGE_API_BASE_URL: &str = "/clip/v2";

#[derive(Debug, Default)]
pub struct HueHueHueBackendConfig {}

#[derive(Debug, Default)]
pub struct HueHueHue {
    _config: HueHueHueBackendConfig,
    bridges: Arc<Mutex<HashMap<String, IpAddr>>>,
    selected_bridge: String,
    client: Client,
    discovery_abort_handle: Option<AbortHandle>,
}

pub struct HueHueHueState(pub Mutex<HueHueHue>);

#[derive(Debug, thiserror::Error)]
pub enum HueHueHueBackendError {
    #[error(transparent)]
    MdnsError(#[from] mdns::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    TokioError(#[from] tokio::task::JoinError),
}

impl serde::Serialize for HueHueHueBackendError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl HueHueHue {
    pub fn with_config(config: impl Into<HueHueHueBackendConfig>) -> HueHueHue {
        HueHueHue {
            _config: config.into(),
            ..Default::default()
        }
    }

    pub fn set_selected_bridge(&mut self, mdns_name: String) -> Result<(), HueHueHueBackendError> {
        self.selected_bridge = mdns_name;

        self.abort_discover()
    }

    pub async fn get_discovered_bridges(&self) -> HashMap<String, IpAddr> {
        self.bridges.lock().await.clone()
    }

    fn get_base_url(&self) -> String {
        format!(
            "{}{}{}",
            HUE_BRIDGE_API_SCHEMA, self.selected_bridge, HUE_BRIDGE_API_BASE_URL
        )
    }

    pub fn abort_discover(&mut self) -> Result<(), HueHueHueBackendError> {
        if let Some(handle) = &self.discovery_abort_handle {
            handle.abort();
        }

        Ok(())
    }

    pub fn discover(&mut self) -> JoinHandle<Result<(), HueHueHueBackendError>> {
        let addrs = self.bridges.clone();
        let discovery_handle = tokio::spawn(async move {
            info!(
                "initiating hue bridge discovery mDNS query service for address \"{}\"...",
                HUE_BRIDGE_SERVICE_NAME
            );
            let stream = mdns::discover::all(
                HUE_BRIDGE_SERVICE_NAME,
                Duration::from_secs(HUE_BRIDGE_SERVICE_QUERY_INTERVAL_SECONDS),
            )?
            .listen();
            pin_mut!(stream);

            while let Some(Ok(resp)) = stream.next().await {
                let addr: Option<(String, IpAddr)> = resp.records().find_map(|r| match r.kind {
                    RecordKind::A(addr) => Some((r.name.clone(), addr.into())),
                    RecordKind::AAAA(addr) => Some((r.name.clone(), addr.into())),
                    _ => None,
                });
                if let Some(addr) = addr {
                    info!(
                        "mDNS response to service name query \"{}\" received from \"{}\" (\"{}\")",
                        HUE_BRIDGE_SERVICE_NAME, &addr.0, &addr.1
                    );
                    let mut addrs = addrs.lock().await;
                    addrs.retain(|_, &mut e| {
                        Into::<IcmpTarget>::into(e).check_availability().is_ok()
                    });
                    addrs.insert(addr.0, addr.1);
                }
            }

            Ok(())
        });

        self.discovery_abort_handle = Some(discovery_handle.abort_handle());

        discovery_handle
    }
}
