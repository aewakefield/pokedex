use pokedex::run;

use reqwest::Url;
use wiremock::MockServer;

pub async fn spawn_app() -> TestApp {
    let addr = ([127, 0, 0, 1], 0);
    let pokeapi_server = MockServer::start().await;
    let translator_server = MockServer::start().await;

    let (addr, server) = run(addr, &pokeapi_server.uri(), &translator_server.uri());
    tokio::spawn(server);

    let address =
        Url::parse(&format!("http://localhost:{}", addr.port())).expect("Failed to parse address");

    TestApp {
        address,
        pokeapi_server,
        translator_server,
    }
}

pub struct TestApp {
    pub address: Url,
    pub pokeapi_server: MockServer,
    pub translator_server: MockServer,
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

pub fn ditto_response(pokeapi_address: &str) -> String {
    format!(
        r#"
{{
  "id": 132,
  "species": {{
    "name": "ditto",
    "url": "{}/pokemon-species/132/"
  }}
}}
"#,
        pokeapi_address
    )
}

pub fn ditto_species_response(pokeapi_address: &str) -> String {
    format!(
        r#"
{{
  "name": "ditto",
  "is_legendary": false,
  "habitat": {{
    "name": "urban",
    "url": "{}/pokemon-habitat/8/"
  }},
  "flavor_text_entries": [
    {{
      "flavor_text": "It can freely recombine its own cellular structure to transform into other life-forms.",
      "language": {{
        "name": "en",
        "url": "{}/language/9/"
      }},
      "version": {{
        "name": "y",
        "url": "{}/version/24/"
      }}
    }}
  ]
}}
"#,
        pokeapi_address, pokeapi_address, pokeapi_address
    )
}

pub fn zubat_response(pokeapi_address: &str) -> String {
    format!(
        r#"
{{
  "id": 41,
  "species": {{
    "name": "zubat",
    "url": "{}/pokemon-species/41/"
  }}
}}
"#,
        pokeapi_address
    )
}

pub fn zubat_species_response(pokeapi_address: &str) -> String {
    format!(
        r#"
{{
  "name": "zubat",
  "is_legendary": false,
  "habitat": {{
    "name": "cave",
    "url": "{}/pokemon-habitat/1/"
  }},
  "flavor_text_entries": [
    {{
      "flavor_text": "Forms colonies in perpetually dark places. Usesultrasonic waves to identify and approach targets.",
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
        pokeapi_address, pokeapi_address, pokeapi_address
    )
}

pub fn translations_response(translated_text: &str) -> String {
    format!(
        r#"
{{
  "contents": {{
    "translated": "{}"
  }}
}}
"#,
        translated_text
    )
}
