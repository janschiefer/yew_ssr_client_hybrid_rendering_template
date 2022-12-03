use yew_hybrid_ssr_client_template::backend_ssr::actix_server::run_actix_server;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");

    run_actix_server().await

}