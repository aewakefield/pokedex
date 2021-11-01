use pokedex::run;

#[tokio::main]
async fn main() {
    let filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info,pokedex=debug,warp=debug".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let addr = ([0, 0, 0, 0], 5000);

    let pokeapi_url = "https://pokeapi.co/api/v2/";
    let translator_url = "https://api.funtranslations.com/translate/";

    let (addr, server) = run(addr, pokeapi_url, translator_url);

    tracing::info!("starting at address {}", addr);

    server.await;
}
