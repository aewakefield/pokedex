use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PokemonSpecies {
    pub name: String,
    pub is_legendary: bool,
    pub habitat: Habitat,
    pub flavor_text_entries: Vec<FlavorText>,
}

#[derive(Deserialize, Debug)]
pub struct Habitat {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct FlavorText {
    pub flavor_text: String,
    pub language: Language,
    pub version: Version,
}

#[derive(Deserialize, Debug)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub url: String,
}
