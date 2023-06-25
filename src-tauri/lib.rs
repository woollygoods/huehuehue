use futures_util::{pin_mut, stream::StreamExt};
use log::info;
use mdns::RecordKind;
use once_cell::sync::Lazy;
use reachable::*;
use std::{
    collections::HashSet, error::Error, fmt::Display, net::IpAddr, sync::Mutex, time::Duration,
};

macro_rules! lazy_lock {
    ($s:expr) => {
        &mut (*$s.lock().unwrap())
    };
}

const HUE_BRIDGE_SERVICE_NAME: &str = "_hue._tcp.local";
static HUE_BRIDGE_IP_ADDRESSES: Lazy<Mutex<HashSet<IpAddr>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Debug)]
pub enum HueHueHueError {}

impl Display for HueHueHueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(clippy::match_single_binding)]
        let desc = match self {
            _ => "unknown error occured in huehuehue",
        };
        write!(f, "{}", desc)
    }
}

impl Error for HueHueHueError {}

pub async fn discover() -> Result<(), Box<dyn Error + Send + Sync>> {
    info!(
        "initiating hue bridge discovery mDNS query service for address \"{}\"...",
        HUE_BRIDGE_SERVICE_NAME
    );
    let stream = mdns::discover::all(HUE_BRIDGE_SERVICE_NAME, Duration::from_secs(15))?.listen();
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
            let addrs = lazy_lock!(HUE_BRIDGE_IP_ADDRESSES);
            for addr in addrs.clone().iter() {
                if Into::<IcmpTarget>::into(*addr)
                    .check_availability()
                    .is_err()
                {
                    addrs.remove(addr);
                }
            }
            addrs.insert(addr);
        }
    }

    Ok(())
}
