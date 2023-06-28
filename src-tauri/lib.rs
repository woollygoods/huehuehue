pub mod transfer;

use futures_util::{pin_mut, stream::StreamExt};
use log::info;
use mdns::RecordKind;
use reachable::*;
use std::{
    collections::HashSet,
    error::Error,
    fmt::Display,
    net::IpAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

const HUE_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";

#[derive(Debug, Default)]
pub struct HueHueHueConfig {}

#[derive(Debug, Default)]
pub struct HueHueHue {
    _config: HueHueHueConfig,
    bridge_ip_addrs: Arc<Mutex<HashSet<IpAddr>>>,
}

pub struct HueHueHueState(pub Mutex<HueHueHue>);

#[derive(Debug)]
pub enum HueHueHueError {
    SynchronizationError,
}

impl Display for HueHueHueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::match_single_binding)]
        let desc = match self {
            HueHueHueError::SynchronizationError => "synchronization error",
        };
        write!(f, "{}", desc)
    }
}

impl Error for HueHueHueError {}

impl HueHueHue {
    pub fn discover(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
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
                    let mut addrs = addrs
                        .lock()
                        .map_err(|_| HueHueHueError::SynchronizationError)?;
                    addrs.retain(|e| Into::<IcmpTarget>::into(*e).check_availability().is_ok());
                    addrs.insert(addr);
                }
            }

            Ok::<(), Box<dyn Error + Send + Sync>>(())
        });

        Ok(())
    }
}
