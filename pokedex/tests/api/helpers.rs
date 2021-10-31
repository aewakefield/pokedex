use pokedex::run;

use reqwest::Url;
use wiremock::MockServer;

pub async fn spawn_app() -> TestApp {
    let addr = ([127, 0, 0, 1], 0);
    let pokeapi_server = MockServer::start().await;

    let (addr, server) = run(addr, &pokeapi_server.uri());
    tokio::spawn(server);

    let address =
        Url::parse(&format!("http://localhost:{}", addr.port())).expect("Failed to parse address");

    TestApp {
        address,
        pokeapi_server,
    }
}

pub struct TestApp {
    pub address: Url,
    pub pokeapi_server: MockServer,
}
