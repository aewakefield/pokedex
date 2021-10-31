use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: usize,
    pub species: Species,
}

#[derive(Deserialize, Debug)]
pub struct Species {
    pub name: String,
    pub url: String,
}
