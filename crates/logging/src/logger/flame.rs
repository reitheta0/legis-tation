use tracing_flame::FlameLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{Registry, fmt};

/// Sets up logging for inferno for testing performance.
pub fn setup_inferno_file() -> impl Drop {
  let fmt_layer = fmt::Layer::default();

  let (flame_layer, _guard) = FlameLayer::with_file("./././log/tracing.folded").unwrap();

  let subscriber = Registry::default().with(fmt_layer).with(flame_layer);

  tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");
  _guard
}
