use crate::helpers::spawn_app;

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

    let template = ResponseTemplate::new(200);
    let template = template.set_body_string(mewtwo_response(&app.pokeapi_server.uri()));
    Mock::given(path("/pokemon/mewtwo"))
        .and(method("GET"))
        .respond_with(template)
        .mount(&app.pokeapi_server)
        .await;

    let template = ResponseTemplate::new(200);
    let template = template.set_body_string(mewtwo_species_response(&app.pokeapi_server.uri()));
    Mock::given(path("/pokemon-species/150/"))
        .and(method("GET"))
        .respond_with(template)
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

pub fn mewtwo_response(pokeapi_address: &str) -> String {
    format!(
        r#"
{{
  "id": 150,
  "species": {{
    "name": "mewtwo",
    "url": "{}/pokemon-species/150/"
  }}
}}
"#,
        pokeapi_address
    )
}

pub fn mewtwo_species_response(pokeapi_address: &str) -> String {
    format!(
        r#"
{{
  "name": "mewtwo",
  "is_legendary": true,
  "habitat": {{
    "name": "rare",
    "url": "{}/pokemon-habitat/5/"
  }},
  "flavor_text_entries": [
    {{
      "flavor_text": "Mewtwo est un Pokémon créé par manipulation génétique.\nCependant, bien que les connaissances scientifiques des\nhumains aient réussi à créer son corps, elles n’ont pas pu\ndoter Mewtwo d’un cœur sensible.",
      "language": {{
        "name": "fr",
        "url": "{}/language/5/"
      }},
      "version": {{
        "name": "omega-ruby",
        "url": "{}/version/25/"
      }}
    }},
    {{
      "flavor_text": "It was created by\na scientist after\nyears of horrific\u000cgene splicing and\nDNA engineering\nexperiments.",
      "language": {{
        "name": "en",
        "url": "{}/language/9/"
      }},
      "version": {{
        "name": "red",
        "url": "{}/version/1/"
      }}
    }}
  ]
}}
"#,
        pokeapi_address, pokeapi_address, pokeapi_address, pokeapi_address, pokeapi_address
    )
}
