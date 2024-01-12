mod router;

use dotenv::dotenv;
use tracing::info;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    info!("Initializing Router");
    let router = router::build_router();
    Ok(router.into())
}
