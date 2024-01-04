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
    // let addr = address::get_address();
    let router = router::build_router();
    // info!("router initialized, now listening on http://{}", addr);


    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // axum::serve(listener, router)
    //     .await
    //     .context("error while starting server")?;

    Ok(router.into())
}
