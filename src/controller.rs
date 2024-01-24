use async_trait::async_trait;
use chrono::{DateTime, Local};
use sd_bus::Message;
use std::collections::HashMap;
use std::sync::Arc;

const SYSTEMD_SERVICE_MANAGER: &str = "org.freedesktop.Systemd1";
const SYSTEMD_OM_IFACE: &str = "org.freedesktop.Systemd1.Manager";
const SYSTEMD_UNIT_IFACE: &str = "org.freedesktop.Systemd1.Unit";

type UnitName = String;
type PropertiesMap = HashMap<String, VariantType>;

#[async_trait]
pub trait Controller {
    async fn poll_loop(&self) -> Result<(), Box<dyn std::error::Error>>;
}

struct ControllersInner {
    bus: Arc<sd_bus::Connection>,
}

struct Controllers {
    inner: ControllersInner,
}

impl Controllers {
    pub async fn new() -> Result<(Self, Arc<sd_bus::Connection>), Box<dyn std::error::Error>> {
        let connection = sd_bus::Connection::system().await?;
        let controllers = Self {
            inner: ControllersInner { bus: Arc::new(connection) },
        };

        Ok((controllers, Arc::clone(&controllers.inner.bus)))
    }
}

#[async_trait]
impl Controller for Controllers {
    async fn poll_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Implement event loop here

        Ok(())
    }
}
