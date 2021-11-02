use crate::helpers::{mewtwo_response, mewtwo_species_response, spawn_app};

use pokeapi::PokemonInfo;

use reqwest::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn get_pokemon_info() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let addr = app
        .address
        .join("pokemon/mewtwo")
        .expect("Failed to make url");

    Mock::given(path("/pokemon/mewtwo"))
        .and(method("GET"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(mewtwo_response(&app.pokeapi_server.uri())),
        )
        .expect(1)
        .named("pokemon GET")
        .mount(&app.pokeapi_server)
        .await;

    Mock::given(path("/pokemon-species/150/"))
        .and(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(mewtwo_species_response(&app.pokeapi_server.uri())),
        )
        .expect(1)
        .named("pokemon-species GET")
        .mount(&app.pokeapi_server)
        .await;

    let expected_info = PokemonInfo {
        name: "mewtwo".to_string(),
        description: Some("It was created by\na scientist after\nyears of horrific\u{000c}gene splicing and\nDNA engineering\nexperiments.".to_string()),
        habitat: "rare".to_string(),
        is_legendary: true,
    };

    // Act
    let response = client
        .get(addr)
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    let actual_info: PokemonInfo = response.json().await.expect("Failed to deserialize json");
    assert_eq!(expected_info, actual_info);
}

#[tokio::test]
async fn get_pokemon_does_not_exist_returns_not_found() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let addr = app
        .address
        .join("pokemon/notapokemon")
        .expect("Failed to make url");

    Mock::given(path("/pokemon/notapokemon"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .named("pokemon GET")
        .mount(&app.pokeapi_server)
        .await;

    // Act
    let response = client
        .get(addr)
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}
