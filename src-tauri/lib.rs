#[macro_use]
pub mod macros;
pub mod endpoints;
pub mod transfer;

use futures_util::{pin_mut, stream::StreamExt};
use log::info;
use mdns::RecordKind;
use reachable::*;
use reqwest::Client;
use std::{collections::HashSet, net::IpAddr, sync::Arc, time::Duration};
use tokio::sync::Mutex;

const HUE_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";
const HUE_BRIDGE_API_BASE_URL: &str = "/clip/v2";

#[derive(Debug, Default)]
pub struct HueHueHueConfig {}

#[derive(Debug, Default)]
pub struct HueHueHue {
    _config: HueHueHueConfig,
    bridge_ip_addrs: Arc<Mutex<HashSet<IpAddr>>>,
    client: Client,
}

pub struct HueHueHueState(pub Mutex<HueHueHue>);

#[derive(Debug, thiserror::Error)]
pub enum HueHueHueError {
    #[error(transparent)]
    MdnsError(#[from] mdns::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl serde::Serialize for HueHueHueError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl HueHueHue {
    fn get_base_url(&self) -> String {
        // TODO: compute the actual base url using the currently selected bridge device
        HUE_BRIDGE_API_BASE_URL.to_string()
    }

    pub fn discover(&self) -> Result<(), HueHueHueError> {
        let addrs = self.bridge_ip_addrs.clone();
        tokio::spawn(async move {
            info!(
                "initiating hue bridge discovery mDNS query service for address \"{}\"...",
                HUE_BRIDGE_SERVICE_NAME
            );
            let stream =
                mdns::discover::all(HUE_BRIDGE_SERVICE_NAME, Duration::from_secs(15))?.listen();
            pin_mut!(stream);

            while let Some(Ok(resp)) = stream.next().await {
                let addr: Option<IpAddr> = resp.records().find_map(|r| match r.kind {
                    RecordKind::A(addr) => Some(addr.into()),
                    RecordKind::AAAA(addr) => Some(addr.into()),
                    _ => None,
                });
                if let Some(addr) = addr {
                    info!(
                        "mDNS response to service name query \"{}\" received from \"{}\"",
                        HUE_BRIDGE_SERVICE_NAME, &addr
                    );
                    let mut addrs = addrs.lock().await;
                    addrs.retain(|e| Into::<IcmpTarget>::into(*e).check_availability().is_ok());
                    addrs.insert(addr);
                }
            }

            Ok::<(), HueHueHueError>(())
        });

        Ok(())
    }
}
