use stefn::{AppError, Broker, EventFactory, EventMetadata};

use crate::entities::events::{Command, Source};

pub async fn send_events(
    command: Command,
    events_broker: &Broker,
    payload: Vec<Vec<u8>>,
) -> Result<u64, AppError> {
    let meta = EventMetadata::new(Source::Core, command, "events".into(), "v1".into());
    let events = EventFactory::new(meta, payload);
    events_broker.send_events(events).await
}
