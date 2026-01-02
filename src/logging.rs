use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(service_name: &str) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug", service_name).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
