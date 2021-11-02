use crate::helpers::{
    ditto_response, ditto_species_response, mewtwo_response, mewtwo_species_response, spawn_app,
    translations_response, zubat_response, zubat_species_response,
};

use pokeapi::PokemonInfo;

use reqwest::StatusCode;
use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

#[tokio::test]
async fn get_pokemon_translated_mewtwo() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let addr = app
        .address
        .join("pokemon/translated/mewtwo")
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

    let translated_text = "translated text";
    Mock::given(path("yoda.json"))
        .and(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(translations_response(translated_text)),
        )
        .expect(1)
        .named("yoda POST")
        .mount(&app.translator_server)
        .await;

    let expected_info = PokemonInfo {
        name: "mewtwo".to_string(),
        description: Some(translated_text.to_string()),
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
async fn get_pokemon_translated_ditto() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let addr = app
        .address
        .join("pokemon/translated/ditto")
        .expect("Failed to make url");

    Mock::given(path("pokemon/ditto"))
        .and(method("GET"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(ditto_response(&app.pokeapi_server.uri())),
        )
        .expect(1)
        .named("pokemon GET")
        .mount(&app.pokeapi_server)
        .await;

    Mock::given(path("pokemon-species/132/"))
        .and(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(ditto_species_response(&app.pokeapi_server.uri())),
        )
        .expect(1)
        .named("pokemon-species GET")
        .mount(&app.pokeapi_server)
        .await;

    let translated_text = "translated text";
    Mock::given(path("shakespeare.json"))
        .and(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(translations_response(translated_text)),
        )
        .expect(1)
        .named("shakespeare POST")
        .mount(&app.translator_server)
        .await;

    let expected_info = PokemonInfo {
        name: "ditto".to_owned(),
        description: Some(translated_text.to_owned()),
        habitat: "urban".to_owned(),
        is_legendary: false,
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
async fn get_pokemon_translated_zubat() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let addr = app
        .address
        .join("pokemon/translated/zubat")
        .expect("Failed to make url");

    Mock::given(path("pokemon/zubat"))
        .and(method("GET"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(zubat_response(&app.pokeapi_server.uri())),
        )
        .expect(1)
        .named("pokemon GET")
        .mount(&app.pokeapi_server)
        .await;

    Mock::given(path("pokemon-species/41/"))
        .and(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(zubat_species_response(&app.pokeapi_server.uri())),
        )
        .expect(1)
        .named("pokemon-species GET")
        .mount(&app.pokeapi_server)
        .await;

    let translated_text = "translated text";
    Mock::given(path("yoda.json"))
        .and(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(translations_response(translated_text)),
        )
        .expect(1)
        .named("yoda POST")
        .mount(&app.translator_server)
        .await;

    let expected_info = PokemonInfo {
        name: "zubat".to_owned(),
        description: Some(translated_text.to_owned()),
        habitat: "cave".to_owned(),
        is_legendary: false,
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
async fn get_pokemon_translated_does_not_exist_returns_not_found() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let addr = app
        .address
        .join("pokemon/translated/notapokemon")
        .expect("Failed to make url");

    Mock::given(path("pokemon/notapokemon"))
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
