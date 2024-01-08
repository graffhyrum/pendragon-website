mod router;

use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio_rs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Initializing Router");
    let router = router::build_router();
    Ok(router.into())
}
