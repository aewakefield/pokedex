mod pokeapi_error;
mod pokeapi_models;

pub use pokeapi_error::PokeapiError;
use pokeapi_models::{
    pokemon::Pokemon,
    pokemon_species::{FlavorText, PokemonSpecies},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Pokeapi {
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl Pokeapi {
    pub fn new(base_url: &str) -> Result<Self, PokeapiError> {
        let base_url = reqwest::Url::parse(base_url)?;

        Ok(Self {
            client: reqwest::Client::new(),
            base_url,
        })
    }

    #[tracing::instrument]
    pub async fn info(&self, pokemon_name: &str) -> Result<PokemonInfo, PokeapiError> {
        let pokemon = self.get_pokemon(pokemon_name).await?;
        let species = self.get_species(&pokemon).await?;

        let info = PokemonInfo {
            name: species.name,
            description: get_description(species.flavor_text_entries),
            habitat: species.habitat.name,
            is_legendary: species.is_legendary,
        };

        Ok(info)
    }

    #[tracing::instrument]
    async fn get_pokemon(&self, pokemon_name: &str) -> Result<Pokemon, PokeapiError> {
        let url = self.base_url.join(&format!("pokemon/{}", pokemon_name))?;
        let pokemon: Pokemon = self.client.get(url).send().await?.json().await?;

        Ok(pokemon)
    }

    #[tracing::instrument]
    async fn get_species(&self, pokemon: &Pokemon) -> Result<PokemonSpecies, PokeapiError> {
        let url = &pokemon.species.url;
        let species: PokemonSpecies = self.client.get(url).send().await?.json().await?;

        Ok(species)
    }
}

#[tracing::instrument]
fn get_description(flavor_text: Vec<FlavorText>) -> Option<String> {
    flavor_text
        .into_iter()
        .find(|flavor| flavor.language.name == "en")
        .map(|flavor| flavor.flavor_text)
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PokemonInfo {
    pub name: String,
    pub description: Option<String>,
    pub habitat: String,
    pub is_legendary: bool,
}
