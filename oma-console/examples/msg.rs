use oma_console::{due_to, success, OmaLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry().with(OmaLayer).init();
    tracing::info!("Welcome");
    tracing::debug!("Hello");
    tracing::info!("I'am fine");
    tracing::warn!("Thank you");
    tracing::error!("and you?");
    due_to!("QAQ");
    success!("PAP");
}
