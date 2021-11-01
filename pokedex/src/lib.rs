use pokeapi::{Pokeapi, PokeapiError};

use core::future::Future;
use serde::Serialize;
use std::net::SocketAddr;
use warp::{http::StatusCode, Filter, Rejection, Reply};

pub fn run(
    addr: impl Into<SocketAddr> + 'static,
    pokeapi_url: &str,
) -> (SocketAddr, impl Future<Output = ()> + 'static) {
    let pokeapi = Pokeapi::new(pokeapi_url).expect("Invalid pokeapi url supplied");

    let pokemon = warp::path!("pokemon" / String)
        .and(warp::get())
        .and(warp::any().map(move || pokeapi.clone()))
        .and_then(info)
        .recover(handle_rejection)
        .with(warp::trace::named("pokemon"));

    let routes = pokemon.with(warp::trace::request());

    warp::serve(routes).bind_ephemeral(addr)
}

#[tracing::instrument]
async fn info(pokemon_name: String, pokeapi: Pokeapi) -> Result<impl warp::Reply, Rejection> {
    let info = pokeapi.info(&pokemon_name).await;

    match info {
        Ok(info) => Ok(warp::reply::json(&info)),
        Err(err) => Err(warp::reject::custom(PokeapiRejection::from(err))),
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(pokeapi_rejection) = err.find::<PokeapiRejection>() {
        let json = warp::reply::json(&ErrorMessage {
            message: pokeapi_rejection.0.to_string(),
        });
        Ok(warp::reply::with_status(
            json,
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Err(err)
    }
}

#[derive(Debug)]
struct PokeapiRejection(PokeapiError);
impl warp::reject::Reject for PokeapiRejection {}
impl From<PokeapiError> for PokeapiRejection {
    fn from(err: PokeapiError) -> Self {
        Self(err)
    }
}

#[derive(Serialize)]
struct ErrorMessage {
    message: String,
}
