use warp::Filter;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/build"]
struct Static;

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let hello_world = warp::path::end()
        .and(warp::get())
        .and(warp_embed::embed(&Static))
        .boxed();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    log::info!("Starting server on http://localhost:3030");
    warp::serve(hello_world.with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
