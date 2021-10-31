use pokedex::run;

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 5000);

    let pokeapi_url = "https://pokeapi.co/api/v2/";

    let (_, server) = run(addr, pokeapi_url);

    server.await;
}
