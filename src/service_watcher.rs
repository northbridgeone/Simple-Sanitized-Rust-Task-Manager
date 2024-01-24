use super::*;
use futures::stream::{self, StreamExt, TryStreamExt};
use sd_bus::MatchRule;
use tokio::sync::broadcast;

const UNIT_STATUS_CHANGED: &str = "StatusChanged";

struct ServiceWatcher {
    bus: sd_bus::Connection,
    tx: broadcast::Sender<UnitName>,
}

impl ServiceWatcher {
    async fn run(bus: &sd_bus::Connection) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, _rx) = broadcast::channel::<UnitName>(10);
        let mut watcher = Self { bus: bus.to_owned(), tx };
        watcher.subscribe().await?;
        watcher.handle_signals().await?;

        Ok(())
    }

    async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.bus
            .add_match(vec![
                ("type".into(), "signal".into(), "sender".into(), "", ""),
                (
                    "destination".into(),
                    SYSTEMD_SERVICE_MANAGER.into(),
                    "path".into(),
                    "/org/freedesktop/Systemd1".into(),
                    UNIT_STATUS_CHANGED.into(),
                ),
            ])
            .await?;

        Ok(())
    }

    async fn handle_signals(&self) -> Result<(), Box<dyn std::error::Error>> {
        stream::iter(self.bus.incoming()).for_each_concurrent(None, |msg| {
            async move {
                let msg = Message::new_method_call(SYSTEMD_OM_IFACE, UNIT_STATUS_CHANGED)?;
                self.process_message(msg).await
            }
        })
        .await;

        Ok(())
    }

    async fn process_message(
        &self,
        message: Message,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Process messages here

        Ok(())
    }
}
